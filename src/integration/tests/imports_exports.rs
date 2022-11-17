use std::{
    io::Write,
    sync::{Arc, Mutex},
};

use candid::{CandidType, Decode, Encode};
mod common;
use common::{create_chat_canister, seed_canister, FakeCommentIter, UpsertCommentParam};
use futures::StreamExt;

use serde::Deserialize;

use crate::common::{create_agent, export_canister_data};

// #[tokio::test(flavor = "multi_thread")]
// async fn test_canister_created() {
//     let agent = create_agent().await;
//     let canister_id = create_chat_canister(&agent).await;
//     match canister_id.clone() {
//         Ok(id) => println!("Canister created with id: {}", id),
//         Err(e) => println!("Error creating canister: {}", e),
//     }
//     assert!(canister_id.is_ok());
// }

// #[tokio::test(flavor = "multi_thread")]
// async fn test_canister_seeding() {
//     let agent = create_agent().await;
//     let canister_id = create_chat_canister(&agent)
//         .await
//         .expect("Canister creation failed");
//     let comments: Vec<UpsertCommentParam> = FakeCommentIter::new().take(100).collect();
//     seed_canister(&agent, &canister_id, comments.clone()).await;
// }

#[tokio::test(flavor = "multi_thread")]
async fn test_canister_csv_exports_simple() {
    const COMMENT_COUNT: usize = 10;

    let agent = create_agent().await;
    let canister_id = create_chat_canister(&agent)
        .await
        .expect("Canister creation failed");
    let comments: Vec<UpsertCommentParam> = FakeCommentIter::new().take(COMMENT_COUNT).collect();
    println!("Seeding canister with {} comments", comments.len());
    seed_canister(&agent, &canister_id, comments.clone()).await;

    #[derive(Deserialize, CandidType)]
    struct ExportChunk {
        data: Vec<u8>,
        next_cursor: Option<u16>,
    }

    #[derive(Deserialize, CandidType)]
    struct ExportParam {
        cursor: u16,
    }


    let data_chunks = export_canister_data(&agent, &canister_id).await;

    let mut file = std::fs::File::create("export_test.csv").expect("File creation failed");
    for chunk in data_chunks {
        file.write_all(&chunk).expect("File write failed");
    }
   
    // count the number of lines in the file
    let count = std::fs::read_to_string("export_test.csv")
        .expect("File read failed")
        .lines()
        .count();

    assert_eq!(count, comments.len());

    // let new_canister_id = create_chat_canister(&agent)
    //     .await
    //     .expect("Canister creation failed");

    // let cloned_data = data.clone();
    // let data_stream = futures::stream::iter(cloned_data.chunks(1000));

    // let results_set = Arc::new(Mutex::new(Vec::<bool>::new()));
    // data_stream
    //     .for_each_concurrent(100, |chunk| async {
    //         let waiter = garcon::Delay::builder()
    //             .timeout(std::time::Duration::from_secs(20))
    //             .build();
    //         let param = chunk.to_vec();
    //         let result = agent
    //             .update(&new_canister_id, "import_comments")
    //             .with_arg(&Encode!(&param).expect("Encode Failure"))
    //             .call_and_wait(waiter)
    //             .await
    //             .expect("Call Failure");

    //         let decoded_result: bool = Decode!(&result, bool).expect("Decode Failure");
    //         results_set.lock().unwrap().push(decoded_result);
    //     })
    //     .await;

    // //check all csv values were pushed successfully
    // assert!(results_set.lock().unwrap().iter().all(|x| *x));

    // //check that data from the new canister matches the original
    // let new_canister_data = export_canister_data(&agent, &new_canister_id).await;
    // let mut file = std::fs::File::create("export_test_2.csv").expect("File creation failed");
    // file.write_all(&new_canister_data).expect("File write failed");
    // assert_eq!(data.len(), new_canister_data.len());

  
}
 