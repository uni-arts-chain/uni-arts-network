#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit="256"]

use sp_std::prelude::*;
use frame_system::{self as system, ensure_root, ensure_signed};

pub use frame_support::{
    construct_runtime, decl_event, decl_module, decl_storage, decl_error,
    dispatch::{DispatchResult},
    ensure, parameter_types, Parameter,
    traits::{
        Currency, LockableCurrency, ExistenceRequirement, Get, Imbalance, KeyOwnerProofSystem, OnUnbalanced,
        Randomness, WithdrawReason, WithdrawReasons
    },
    weights::{
        constants::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_PER_SECOND},
        DispatchInfo, GetDispatchInfo, IdentityFee, Pays, PostDispatchInfo, Weight,
        WeightToFeePolynomial,
    },
    IsSubType, StorageValue, debug,
};
use pallet_timestamp as timestamp;
mod types;

pub use crate::types::*;

/// Constants

/// The greatest valid depth of a category.
/// The depth of a root category is 0.
const MAX_CATEGORY_DEPTH: u16 = 3;

/// Error messages for dispatchables
const ERROR_FORUM_SUDO_NOT_SET: &str = "Forum sudo not set.";
const ERROR_ORIGIN_NOT_FORUM_SUDO: &str = "Origin not forum sudo.";
const ERROR_CATEGORY_TITLE_TOO_SHORT: &str = "Category title too short.";
const ERROR_CATEGORY_TITLE_TOO_LONG: &str = "Category title too long.";
const ERROR_CATEGORY_DESCRIPTION_TOO_SHORT: &str = "Category description too long.";
const ERROR_CATEGORY_DESCRIPTION_TOO_LONG: &str = "Category description too long.";
const ERROR_ANCESTOR_CATEGORY_IMMUTABLE: &str =
    "Ancestor category immutable, i.e. deleted or archived";
const ERROR_MAX_VALID_CATEGORY_DEPTH_EXCEEDED: &str = "Maximum valid category depth exceeded.";
const ERROR_CATEGORY_DOES_NOT_EXIST: &str = "Category does not exist.";
const ERROR_NOT_FORUM_USER: &str = "Not forum user.";
const ERROR_THREAD_TITLE_TOO_SHORT: &str = "Thread title too short.";
const ERROR_THREAD_TITLE_TOO_LONG: &str = "Thread title too long.";
const ERROR_POST_TEXT_TOO_SHORT: &str = "Post text too short.";
const ERROR_POST_TEXT_TOO_LONG: &str = "Post too long.";
const ERROR_THREAD_DOES_NOT_EXIST: &str = "Thread does not exist";
const ERROR_THREAD_MODERATION_RATIONALE_TOO_SHORT: &str = "Thread moderation rationale too short.";
const ERROR_THREAD_MODERATION_RATIONALE_TOO_LONG: &str = "Thread moderation rationale too long.";
const ERROR_THREAD_ALREADY_MODERATED: &str = "Thread already moderated.";
const ERROR_THREAD_MODERATED: &str = "Thread is moderated.";
const ERROR_POST_DOES_NOT_EXIST: &str = "Post does not exist.";
const ERROR_ACCOUNT_DOES_NOT_MATCH_POST_AUTHOR: &str = "Account does not match post author.";
const ERROR_POST_MODERATED: &str = "Post is moderated.";
const ERROR_POST_MODERATION_RATIONALE_TOO_SHORT: &str = "Post moderation rationale too short.";
const ERROR_POST_MODERATION_RATIONALE_TOO_LONG: &str = "Post moderation rationale too long.";
const ERROR_CATEGORY_NOT_BEING_UPDATED: &str = "Category not being updated.";
const ERROR_CATEGORY_CANNOT_BE_UNARCHIVED_WHEN_DELETED: &str =
    "Category cannot be unarchived when deleted.";

pub trait Trait: system::Trait + timestamp::Trait + Sized {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

    type MembershipRegistry: ForumUserRegistry<Self::AccountId>;
}

decl_storage! {
    trait Store for Module<T: Trait> as Forum {

        /// Map category identifier to corresponding category.
        pub CategoryById get(fn category_by_id): map hasher(identity) CategoryId => Category<T::BlockNumber, T::Moment, T::AccountId>;

        /// Category identifier value to be used for the next Category created.
        pub NextCategoryId get(fn next_category_id): CategoryId;

        /// Map thread identifier to corresponding thread.
        pub ThreadById get(fn thread_by_id): map hasher(identity) ThreadId => Thread<T::BlockNumber, T::Moment, T::AccountId>;

        /// Thread identifier value to be used for next Thread in threadById.
        pub NextThreadId get(fn next_thread_id): ThreadId;

        /// Map post identifier to corresponding post.
        pub PostById get(fn post_by_id): map hasher(identity) PostId => Post<T::BlockNumber, T::Moment, T::AccountId>;

        /// Post identifier value to be used for for next post created.
        pub NextPostId get(fn next_post_id): PostId;

        /// Account of forum sudo.
        pub ForumSudo get(fn forum_sudo): Option<T::AccountId>;

        /// Input constraints
        /// These are all forward looking, that is they are enforced on all
        /// future calls.
        pub CategoryTitleConstraint get(fn category_title_constraint): InputValidationLengthConstraint;
        pub CategoryDescriptionConstraint get(fn category_description_constraint): InputValidationLengthConstraint;
        pub ThreadTitleConstraint get(fn thread_title_constraint): InputValidationLengthConstraint;
        pub PostTextConstraint get(fn post_text_constraint): InputValidationLengthConstraint;
        pub ThreadModerationRationaleConstraint get(fn thread_moderation_rationale_constraint): InputValidationLengthConstraint;
        pub PostModerationRationaleConstraint get(fn post_moderation_rationale_constraint): InputValidationLengthConstraint;
    }
}

decl_event!(
    pub enum Event<T>
    where
        <T as system::Trait>::AccountId,
    {
        /// A category was introduced
        CategoryCreated(CategoryId),

        /// A category with given id was updated.
        /// The second argument reflects the new archival status of the category, if changed.
        /// The third argument reflects the new deletion status of the category, if changed.
        CategoryUpdated(CategoryId, Option<bool>, Option<bool>),

        /// A thread with given id was created.
        ThreadCreated(ThreadId),

        /// A thread with given id was moderated.
        ThreadModerated(ThreadId),

        /// Post with given id was created.
        PostAdded(PostId),

        /// Post with givne id was moderated.
        PostModerated(PostId),

        /// Post with given id had its text updated.
        /// The second argument reflects the number of total edits when the text update occurs.
        PostTextUpdated(PostId, u64),

        /// Given account was set as forum sudo.
        ForumSudoSet(Option<AccountId>, Option<AccountId>),
    }
);

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        fn deposit_event() = default;

        #[weight = 0]
        fn set_forum_sudo(origin, new_forum_sudo: Option<T::AccountId>) -> DispatchResult {
            ensure_root(origin)?;

            // Hold on to old value
            let old_forum_sudo = <ForumSudo<T>>::get().clone();

            // Update forum sudo
            match new_forum_sudo.clone() {
                Some(account_id) => <ForumSudo<T>>::put(account_id),
                None => <ForumSudo<T>>::kill()
            };

            // Generate event
            Self::deposit_event(RawEvent::ForumSudoSet(old_forum_sudo, new_forum_sudo));

            // All good.
            Ok(())
        }

        /// Add a new category.
        #[weight = 0]
        fn create_category(origin, parent: Option<CategoryId>, title: Vec<u8>, description: Vec<u8>) -> DispatchResult {

            // Check that its a valid signature
            let who = ensure_signed(origin)?;

            // Not signed by forum SUDO
            Self::ensure_is_forum_sudo(&who)?;

            // Validate title
            Self::ensure_category_title_is_valid(&title)?;

            // Validate description
            Self::ensure_category_description_is_valid(&description)?;

            // Position in parent field value for new category
            let mut position_in_parent_category_field = None;

            // If not root, then check that we can create in parent category
            if let Some(parent_category_id) = parent {

                let category_tree_path = Self::ensure_valid_category_and_build_category_tree_path(parent_category_id)?;

                // Can we mutate in this category?
                Self::ensure_can_add_subcategory_path_leaf(&category_tree_path)?;

                /*
                 * Here we are safe to mutate
                 */

                // Increment number of subcategories to reflect this new category being
                // added as a child
                <CategoryById<T>>::mutate(parent_category_id, |c| {
                    c.num_direct_subcategories += 1;
                });

                // Set `position_in_parent_category_field`
                let parent_category = category_tree_path.first().unwrap();

                position_in_parent_category_field = Some(ChildPositionInParentCategory{
                    parent_id: parent_category_id,
                    child_nr_in_parent_category: parent_category.num_direct_subcategories
                });

            }

            let next_category_id = NextCategoryId::get();

            // Create new category
            let new_category = Category {
                id : next_category_id,
                title : title.clone(),
                description: description.clone(),
                created_at : Self::current_block_and_time(),
                deleted: false,
                archived: false,
                num_direct_subcategories: 0,
                num_direct_unmoderated_threads: 0,
                num_direct_moderated_threads: 0,
                position_in_parent_category: position_in_parent_category_field,
                moderator_id: who
            };

            // Insert category in map
            <CategoryById<T>>::insert(new_category.id, new_category);

            // Update other things
            NextCategoryId::put(next_category_id + 1);

            // Generate event
            Self::deposit_event(RawEvent::CategoryCreated(next_category_id));

            Ok(())
        }

        /// Update category
        #[weight = 0]
        fn update_category(origin, category_id: CategoryId, new_archival_status: Option<bool>, new_deletion_status: Option<bool>) -> DispatchResult {

            // Check that its a valid signature
            let who = ensure_signed(origin)?;

            // Not signed by forum SUDO
            Self::ensure_is_forum_sudo(&who)?;

            // Make sure something is actually being changed
            ensure!(
                new_archival_status.is_some() || new_deletion_status.is_some(),
                ERROR_CATEGORY_NOT_BEING_UPDATED
            );

            // Get path from parent to root of category tree.
            let category_tree_path = Self::ensure_valid_category_and_build_category_tree_path(category_id)?;

            // When we are dealing with a non-root category, we
            // must ensure mutability of our category by traversing to
            // root.
            if category_tree_path.len() > 1  {

                // We must skip checking category itself.
                // NB: This is kind of hacky way to avoid last element,
                // something clearn can be done later.
                let mut path_to_check = category_tree_path.clone();
                path_to_check.remove(0);

                Self::ensure_can_mutate_in_path_leaf(&path_to_check)?;
            }

            // If the category itself is already deleted, then this
            // update *must* simultaneously do an undelete, otherwise it is blocked,
            // as we do not permit unarchiving a deleted category. Doing
            // a simultanous undelete and unarchive is accepted.

            let category = <CategoryById<T>>::get(category_id);

            ensure!(
                !category.deleted || (new_deletion_status == Some(false)),
                ERROR_CATEGORY_CANNOT_BE_UNARCHIVED_WHEN_DELETED
            );

            // Mutate category, and set possible new change parameters

            <CategoryById<T>>::mutate(category_id, |c| {

                if let Some(archived) = new_archival_status {
                    c.archived = archived;
                }

                if let Some(deleted) = new_deletion_status {
                    c.deleted = deleted;
                }
            });

            // Generate event
            Self::deposit_event(RawEvent::CategoryUpdated(category_id, new_archival_status, new_deletion_status));

            Ok(())
        }

        /// Create new thread in category
        #[weight = 0]
        fn create_thread(origin, category_id: CategoryId, title: Vec<u8>, text: Vec<u8>) -> DispatchResult {

            // Check that its a valid signature
            let who = ensure_signed(origin)?;

            // Check that account is forum member
            Self::ensure_is_forum_member(&who)?;

            // Get path from parent to root of category tree.
            let category_tree_path = Self::ensure_valid_category_and_build_category_tree_path(category_id)?;

            // No ancestor is blocking us doing mutation in this category
            Self::ensure_can_mutate_in_path_leaf(&category_tree_path)?;

            // Validate title
            Self::ensure_thread_title_is_valid(&title)?;

            // Validate post text
            Self::ensure_post_text_is_valid(&text)?;

            // Add thread
            let thread = Self::add_new_thread(category_id, &title, &who);

            // Add inital post to thread
            Self::add_new_post(thread.id, &text, &who);

            // Generate event
            Self::deposit_event(RawEvent::ThreadCreated(thread.id));

            Ok(())
        }

        /// Moderate thread
        #[weight = 0]
        fn moderate_thread(origin, thread_id: ThreadId, rationale: Vec<u8>) -> DispatchResult {

            // Check that its a valid signature
            let who = ensure_signed(origin)?;

            // Signed by forum SUDO
            Self::ensure_is_forum_sudo(&who)?;

            // Get thread
            let mut thread = Self::ensure_thread_exists(&thread_id)?;

            // Thread is not already moderated
            ensure!(thread.moderation.is_none(), ERROR_THREAD_ALREADY_MODERATED);

            // Rationale valid
            Self::ensure_thread_moderation_rationale_is_valid(&rationale)?;

            // Can mutate in corresponding category
            let path = Self::build_category_tree_path(thread.category_id);

            // Path must be non-empty, as category id is from thread in state
            assert!(!path.is_empty());

            Self::ensure_can_mutate_in_path_leaf(&path)?;

            // Add moderation to thread
            thread.moderation = Some(ModerationAction {
                moderated_at: Self::current_block_and_time(),
                moderator_id: who,
                rationale: rationale.clone()
            });

            <ThreadById<T>>::insert(thread_id, thread.clone());

            // Update moderation/umoderation count of corresponding category
            <CategoryById<T>>::mutate(thread.category_id, |category| {
                category.num_direct_unmoderated_threads -= 1;
                category.num_direct_moderated_threads += 1;
            });

            // Generate event
            Self::deposit_event(RawEvent::ThreadModerated(thread_id));

            Ok(())
        }

        /// Edit post text
        #[weight = 0]
        fn add_post(origin, thread_id: ThreadId, text: Vec<u8>) -> DispatchResult {

            // Check that its a valid signature
            let who = ensure_signed(origin)?;

            // Check that account is forum member
            Self::ensure_is_forum_member(&who)?;

            // Validate post text
            Self::ensure_post_text_is_valid(&text)?;

            // Make sure thread exists and is mutable
            let thread = Self::ensure_thread_is_mutable(&thread_id)?;

            // Get path from parent to root of category tree.
            let category_tree_path = Self::ensure_valid_category_and_build_category_tree_path(thread.category_id)?;

            // No ancestor is blocking us doing mutation in this category
            Self::ensure_can_mutate_in_path_leaf(&category_tree_path)?;

            let post = Self::add_new_post(thread_id, &text, &who);

            // Generate event
            Self::deposit_event(RawEvent::PostAdded(post.id));

            Ok(())
        }

        /// Edit post text
        #[weight = 0]
        fn edit_post_text(origin, post_id: PostId, new_text: Vec<u8>) -> DispatchResult {

            /* Edit spec.
              - forum member guard missing
              - check that both post and thread and category are mutable
            */

            // Check that its a valid signature
            let who = ensure_signed(origin)?;

            // Check that account is forum member
            Self::ensure_is_forum_member(&who)?;

            // Validate post text
            Self::ensure_post_text_is_valid(&new_text)?;

            // Make sure there exists a mutable post with post id `post_id`
            let post = Self::ensure_post_is_mutable(&post_id)?;

            // Signer does not match creator of post with identifier postId
            ensure!(post.author_id == who, ERROR_ACCOUNT_DOES_NOT_MATCH_POST_AUTHOR);

            <PostById<T>>::mutate(post_id, |p| {

                let expired_post_text = PostTextChange {
                    expired_at: Self::current_block_and_time(),
                    text: post.current_text.clone()
                };

                // Set current text to new text
                p.current_text = new_text;

                // Copy current text to history of expired texts
                p.text_change_history.push(expired_post_text);
            });

            // Generate event
            Self::deposit_event(RawEvent::PostTextUpdated(post.id, post.text_change_history.len() as u64));

            Ok(())
        }

        /// Moderate post
        #[weight = 0]
        fn moderate_post(origin, post_id: PostId, rationale: Vec<u8>) -> DispatchResult {

            // Check that its a valid signature
            let who = ensure_signed(origin)?;

            // Signed by forum SUDO
            Self::ensure_is_forum_sudo(&who)?;

            // Make sure post exists and is mutable
            let post = Self::ensure_post_is_mutable(&post_id)?;

            Self::ensure_post_moderation_rationale_is_valid(&rationale)?;

            // Update moderation action on post
            let moderation_action = ModerationAction{
                moderated_at: Self::current_block_and_time(),
                moderator_id: who,
                rationale: rationale.clone()
            };

            <PostById<T>>::mutate(post_id, |p| {
                p.moderation = Some(moderation_action);
            });

            // Update moderated and unmoderated post count of corresponding thread
            <ThreadById<T>>::mutate(post.thread_id, |t| {
                t.num_unmoderated_posts -= 1;
                t.num_moderated_posts += 1;
            });

            // Generate event
            Self::deposit_event(RawEvent::PostModerated(post.id));

            Ok(())
        }

    }
}

impl<T: Trait> Module<T> {
    fn ensure_category_title_is_valid(title: &Vec<u8>) -> DispatchResult {
        CategoryTitleConstraint::get().ensure_valid(
            title.len(),
            ERROR_CATEGORY_TITLE_TOO_SHORT,
            ERROR_CATEGORY_TITLE_TOO_LONG,
        )
    }

    fn ensure_category_description_is_valid(description: &Vec<u8>) -> DispatchResult {
        CategoryDescriptionConstraint::get().ensure_valid(
            description.len(),
            ERROR_CATEGORY_DESCRIPTION_TOO_SHORT,
            ERROR_CATEGORY_DESCRIPTION_TOO_LONG,
        )
    }

    fn ensure_thread_moderation_rationale_is_valid(rationale: &Vec<u8>) -> DispatchResult {
        ThreadModerationRationaleConstraint::get().ensure_valid(
            rationale.len(),
            ERROR_THREAD_MODERATION_RATIONALE_TOO_SHORT,
            ERROR_THREAD_MODERATION_RATIONALE_TOO_LONG,
        )
    }

    fn ensure_thread_title_is_valid(title: &Vec<u8>) -> DispatchResult {
        ThreadTitleConstraint::get().ensure_valid(
            title.len(),
            ERROR_THREAD_TITLE_TOO_SHORT,
            ERROR_THREAD_TITLE_TOO_LONG,
        )
    }

    fn ensure_post_text_is_valid(text: &Vec<u8>) -> DispatchResult {
        PostTextConstraint::get().ensure_valid(
            text.len(),
            ERROR_POST_TEXT_TOO_SHORT,
            ERROR_POST_TEXT_TOO_LONG,
        )
    }

    fn ensure_post_moderation_rationale_is_valid(rationale: &Vec<u8>) -> DispatchResult {
        PostModerationRationaleConstraint::get().ensure_valid(
            rationale.len(),
            ERROR_POST_MODERATION_RATIONALE_TOO_SHORT,
            ERROR_POST_MODERATION_RATIONALE_TOO_LONG,
        )
    }

    fn current_block_and_time() -> BlockchainTimestamp<T::BlockNumber, T::Moment> {
        BlockchainTimestamp {
            block: <system::Module<T>>::block_number(),
            time: <timestamp::Module<T>>::now(),
        }
    }

    fn ensure_post_is_mutable(
        post_id: &PostId,
    ) -> Result<Post<T::BlockNumber, T::Moment, T::AccountId>, &'static str> {
        // Make sure post exists
        let post = Self::ensure_post_exists(post_id)?;

        // and is unmoderated
        ensure!(post.moderation.is_none(), ERROR_POST_MODERATED);

        // and make sure thread is mutable
        Self::ensure_thread_is_mutable(&post.thread_id)?;

        Ok(post)
    }

    fn ensure_post_exists(
        post_id: &PostId,
    ) -> Result<Post<T::BlockNumber, T::Moment, T::AccountId>, &'static str> {
        if <PostById<T>>::contains_key(post_id) {
            Ok(<PostById<T>>::get(post_id))
        } else {
            Err(ERROR_POST_DOES_NOT_EXIST)
        }
    }

    fn ensure_thread_is_mutable(
        thread_id: &ThreadId,
    ) -> Result<Thread<T::BlockNumber, T::Moment, T::AccountId>, &'static str> {
        // Make sure thread exists
        let thread = Self::ensure_thread_exists(&thread_id)?;

        // and is unmoderated
        ensure!(thread.moderation.is_none(), ERROR_THREAD_MODERATED);

        // and corresponding category is mutable
        Self::ensure_catgory_is_mutable(thread.category_id)?;

        Ok(thread)
    }

    fn ensure_thread_exists(
        thread_id: &ThreadId,
    ) -> Result<Thread<T::BlockNumber, T::Moment, T::AccountId>, &'static str> {
        if <ThreadById<T>>::contains_key(thread_id) {
            Ok(<ThreadById<T>>::get(thread_id))
        } else {
            Err(ERROR_THREAD_DOES_NOT_EXIST)
        }
    }

    fn ensure_forum_sudo_set() -> Result<T::AccountId, &'static str> {
        match <ForumSudo<T>>::get() {
            Some(account_id) => Ok(account_id),
            None => Err(ERROR_FORUM_SUDO_NOT_SET),
        }
    }

    fn ensure_is_forum_sudo(account_id: &T::AccountId) -> DispatchResult {
        let forum_sudo_account = Self::ensure_forum_sudo_set()?;

        ensure!(
            *account_id == forum_sudo_account,
            ERROR_ORIGIN_NOT_FORUM_SUDO
        );
        Ok(())
    }

    fn ensure_is_forum_member(
        account_id: &T::AccountId,
    ) -> Result<ForumUser<T::AccountId>, &'static str> {
        let forum_user_query = T::MembershipRegistry::get_forum_user(account_id);

        if let Some(forum_user) = forum_user_query {
            Ok(forum_user)
        } else {
            Err(ERROR_NOT_FORUM_USER)
        }
    }

    fn ensure_catgory_is_mutable(category_id: CategoryId) -> DispatchResult {
        let category_tree_path = Self::build_category_tree_path(category_id);

        Self::ensure_can_mutate_in_path_leaf(&category_tree_path)
    }

    fn ensure_can_mutate_in_path_leaf(
        category_tree_path: &CategoryTreePath<T::BlockNumber, T::Moment, T::AccountId>,
    ) -> DispatchResult {
        // Is parent category directly or indirectly deleted or archived category
        ensure!(
            !category_tree_path.iter().any(
                |c: &Category<T::BlockNumber, T::Moment, T::AccountId>| c.deleted || c.archived
            ),
            ERROR_ANCESTOR_CATEGORY_IMMUTABLE
        );

        Ok(())
    }

    fn ensure_can_add_subcategory_path_leaf(
        category_tree_path: &CategoryTreePath<T::BlockNumber, T::Moment, T::AccountId>,
    ) -> DispatchResult {
        Self::ensure_can_mutate_in_path_leaf(category_tree_path)?;

        // Does adding a new category exceed maximum depth
        let depth_of_new_category = 1 + 1 + category_tree_path.len();

        ensure!(
            depth_of_new_category <= MAX_CATEGORY_DEPTH as usize,
            ERROR_MAX_VALID_CATEGORY_DEPTH_EXCEEDED
        );

        Ok(())
    }

    fn ensure_valid_category_and_build_category_tree_path(
        category_id: CategoryId,
    ) -> Result<CategoryTreePath<T::BlockNumber, T::Moment, T::AccountId>, &'static str> {
        ensure!(
            <CategoryById<T>>::contains_key(&category_id),
            ERROR_CATEGORY_DOES_NOT_EXIST
        );

        // Get path from parent to root of category tree.
        let category_tree_path = Self::build_category_tree_path(category_id);

        assert!(category_tree_path.len() > 0);

        Ok(category_tree_path)
    }

    /// Builds path and populates in `path`.
    /// Requires that `category_id` is valid
    fn build_category_tree_path(
        category_id: CategoryId,
    ) -> CategoryTreePath<T::BlockNumber, T::Moment, T::AccountId> {
        // Get path from parent to root of category tree.
        let mut category_tree_path = vec![];

        Self::_build_category_tree_path(category_id, &mut category_tree_path);

        category_tree_path
    }

    /// Builds path and populates in `path`.
    /// Requires that `category_id` is valid
    fn _build_category_tree_path(
        category_id: CategoryId,
        path: &mut CategoryTreePath<T::BlockNumber, T::Moment, T::AccountId>,
    ) {
        // Grab category
        let category = <CategoryById<T>>::get(category_id);

        // Copy out position_in_parent_category
        let position_in_parent_category_field = category.position_in_parent_category.clone();

        // Add category to path container
        path.push(category);

        // Make recursive call on parent if we are not at root
        if let Some(child_position_in_parent) = position_in_parent_category_field {
            assert!(<CategoryById<T>>::contains_key(
                &child_position_in_parent.parent_id
            ));

            Self::_build_category_tree_path(child_position_in_parent.parent_id, path);
        }
    }

    fn add_new_thread(
        category_id: CategoryId,
        title: &Vec<u8>,
        author_id: &T::AccountId,
    ) -> Thread<T::BlockNumber, T::Moment, T::AccountId> {
        // Get category
        let category = <CategoryById<T>>::get(category_id);

        // Create and add new thread
        let new_thread_id = NextThreadId::get();

        let new_thread = Thread {
            id: new_thread_id,
            title: title.clone(),
            category_id: category_id,
            nr_in_category: category.num_threads_created() + 1,
            moderation: None,
            num_unmoderated_posts: 0,
            num_moderated_posts: 0,
            created_at: Self::current_block_and_time(),
            author_id: author_id.clone(),
        };

        // Store thread
        <ThreadById<T>>::insert(new_thread_id, new_thread.clone());

        // Update next thread id
        NextThreadId::mutate(|n| {
            *n += 1;
        });

        // Update unmoderated thread count in corresponding category
        <CategoryById<T>>::mutate(category_id, |c| {
            c.num_direct_unmoderated_threads += 1;
        });

        new_thread
    }

    /// Creates and ads a new post ot the given thread, and makes all required state updates
    /// `thread_id` must be valid
    fn add_new_post(
        thread_id: ThreadId,
        text: &Vec<u8>,
        author_id: &T::AccountId,
    ) -> Post<T::BlockNumber, T::Moment, T::AccountId> {
        // Get thread
        let thread = <ThreadById<T>>::get(thread_id);

        // Make and add initial post
        let new_post_id = NextPostId::get();

        let new_post = Post {
            id: new_post_id,
            thread_id: thread_id,
            nr_in_thread: thread.num_posts_ever_created() + 1,
            current_text: text.clone(),
            moderation: None,
            text_change_history: vec![],
            created_at: Self::current_block_and_time(),
            author_id: author_id.clone(),
        };

        // Store post
        <PostById<T>>::insert(new_post_id, new_post.clone());

        // Update next post id
        NextPostId::mutate(|n| {
            *n += 1;
        });

        // Update unmoderated post count of thread
        <ThreadById<T>>::mutate(thread_id, |t| {
            t.num_unmoderated_posts += 1;
        });

        new_post
    }
}