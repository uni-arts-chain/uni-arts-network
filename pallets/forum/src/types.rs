use codec::{Decode, Encode};
use sp_std::prelude::Vec;
use serde::{Deserialize, Serialize};
use sp_runtime::DispatchError;
use frame_support::dispatch::DispatchResult;


/// Length constraint for input validation
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq)]
pub struct InputValidationLengthConstraint {
    /// Minimum length
    pub min: u16,

    /// Difference between minimum length and max length.
    /// While having max would have been more direct, this
    /// way makes max < min unrepresentable semantically,
    /// which is safer.
    pub max_min_diff: u16,
}

impl InputValidationLengthConstraint {
    /// Helper for computing max
    pub fn max(&self) -> u16 {
        self.min + self.max_min_diff
    }

    pub fn ensure_valid(
        &self,
        len: usize,
        too_short_msg: &'static str,
        too_long_msg: &'static str,
    ) -> DispatchResult {
        let length = len as u16;
        if length < self.min {
            DispatchResult::Err(DispatchError::Other(too_short_msg))
        } else if length > self.max() {
            DispatchResult::Err(DispatchError::Other(too_long_msg))
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ForumUser<AccountId> {
    /// Identifier of user
    pub id: AccountId, // In the future one could add things like
    // - updating post count of a user
    // - updating status (e.g. hero, new, etc.)
    //
}

/// Represents a regsitry of `ForumUser` instances.
pub trait ForumUserRegistry<AccountId> {
    fn get_forum_user(id: &AccountId) -> Option<ForumUser<AccountId>>;
}

/// Convenient composite time stamp
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq)]
pub struct BlockchainTimestamp<BlockNumber, Moment> {
    pub block: BlockNumber,
    pub time: Moment,
}

/// Represents a moderation outcome applied to a post or a thread.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq)]
pub struct ModerationAction<BlockNumber, Moment, AccountId> {
    /// When action occured.
    pub moderated_at: BlockchainTimestamp<BlockNumber, Moment>,

    /// Account forum sudo which acted.
    pub moderator_id: AccountId,

    /// Moderation rationale
    pub rationale: Vec<u8>,
}

/// Represents a revision of the text of a Post
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq)]
pub struct PostTextChange<BlockNumber, Moment> {
    /// When this expiration occured
    pub expired_at: BlockchainTimestamp<BlockNumber, Moment>,

    /// Text that expired
    pub text: Vec<u8>,
}

/// Represents a post identifier
pub type PostId = u64;

/// Represents a thread post
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq)]
pub struct Post<BlockNumber, Moment, AccountId> {
    /// Post identifier
    pub id: PostId,

    /// Id of thread to which this post corresponds.
    pub thread_id: ThreadId,

    /// The post number of this post in its thread, i.e. total number of posts added (including this)
    /// to a thread when it was added.
    /// Is needed to give light clients assurance about getting all posts in a given range,
    // `created_at` is not sufficient.
    /// Starts at 1 for first post in thread.
    pub nr_in_thread: u32,

    /// Current text of post
    pub current_text: Vec<u8>,

    /// Possible moderation of this post
    pub moderation: Option<ModerationAction<BlockNumber, Moment, AccountId>>,

    /// Edits of post ordered chronologically by edit time.
    pub text_change_history: Vec<PostTextChange<BlockNumber, Moment>>,

    /// When post was submitted.
    pub created_at: BlockchainTimestamp<BlockNumber, Moment>,

    /// Author of post.
    pub author_id: AccountId,
}

/// Represents a thread identifier
pub type ThreadId = u64;

/// Represents a thread
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq)]
pub struct Thread<BlockNumber, Moment, AccountId> {
    /// Thread identifier
    pub id: ThreadId,

    /// Title
    pub title: Vec<u8>,

    /// Category in which this thread lives
    pub category_id: CategoryId,

    /// The thread number of this thread in its category, i.e. total number of thread added (including this)
    /// to a category when it was added.
    /// Is needed to give light clients assurance about getting all threads in a given range,
    /// `created_at` is not sufficient.
    /// Starts at 1 for first thread in category.
    pub nr_in_category: u32,

    /// Possible moderation of this thread
    pub moderation: Option<ModerationAction<BlockNumber, Moment, AccountId>>,

    /// Number of unmoderated and moderated posts in this thread.
    /// The sum of these two only increases, and former is incremented
    /// for each new post added to this thread. A new post is added
    /// with a `nr_in_thread` equal to this sum
    ///
    /// When there is a moderation
    /// of a post, the variables are incremented and decremented, respectively.
    ///
    /// These values are vital for light clients, in order to validate that they are
    /// not being censored from posts in a thread.
    pub num_unmoderated_posts: u32,
    pub num_moderated_posts: u32,

    /// When thread was established.
    pub created_at: BlockchainTimestamp<BlockNumber, Moment>,

    /// Author of post.
    pub author_id: AccountId,
}

impl<BlockNumber, Moment, AccountId> Thread<BlockNumber, Moment, AccountId> {
    pub fn num_posts_ever_created(&self) -> u32 {
        self.num_unmoderated_posts + self.num_moderated_posts
    }
}

/// Represents a category identifier
pub type CategoryId = u64;

/// Represents
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq)]
pub struct ChildPositionInParentCategory {
    /// Id of parent category
    pub parent_id: CategoryId,

    /// Nr of the child in the parent
    /// Starts at 1
    pub child_nr_in_parent_category: u32,
}

/// Represents a category
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq)]
pub struct Category<BlockNumber, Moment, AccountId> {
    /// Category identifier
    pub id: CategoryId,

    /// Title
    pub title: Vec<u8>,

    /// Description
    pub description: Vec<u8>,

    /// When category was established.
    pub created_at: BlockchainTimestamp<BlockNumber, Moment>,

    /// Whether category is deleted.
    pub deleted: bool,

    /// Whether category is archived.
    pub archived: bool,

    /// Number of subcategories (deleted, archived or neither),
    /// unmoderated threads and moderated threads, _directly_ in this category.
    ///
    /// As noted, the first is unaffected by any change in state of direct subcategory.
    ///
    /// The sum of the latter two only increases, and former is incremented
    /// for each new thread added to this category. A new thread is added
    /// with a `nr_in_category` equal to this sum.
    ///
    /// When there is a moderation
    /// of a thread, the variables are incremented and decremented, respectively.
    ///
    /// These values are vital for light clients, in order to validate that they are
    /// not being censored from subcategories or threads in a category.
    pub num_direct_subcategories: u32,
    pub num_direct_unmoderated_threads: u32,
    pub num_direct_moderated_threads: u32,

    /// Position as child in parent, if present, otherwise this category is a root category
    pub position_in_parent_category: Option<ChildPositionInParentCategory>,

    /// Account of the moderator which created category.
    pub moderator_id: AccountId,
}

impl<BlockNumber, Moment, AccountId> Category<BlockNumber, Moment, AccountId> {
    pub fn num_threads_created(&self) -> u32 {
        self.num_direct_unmoderated_threads + self.num_direct_moderated_threads
    }
}

/// Represents a sequence of categories which have child-parent relatioonship
/// where last element is final ancestor, or root, in the context of the category tree.
pub type CategoryTreePath<BlockNumber, Moment, AccountId> = Vec<Category<BlockNumber, Moment, AccountId>>;