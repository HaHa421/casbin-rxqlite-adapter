use super::*;

#[test]
fn test_create() {
  let rt = Runtime::new().unwrap();
  let _ = rt.block_on(async {
        //const QUERY: &str ="SELECT name,birth_date from _test_user_ where name = ?";
    let /*mut*/ tm = TestManager::new("test_create",3,None);
    tm.wait_for_cluster_established(1, 60).await.unwrap();
    
    let http_addr = tm.instances.get(&1).unwrap().http_addr.clone();
    crate::adapter::tests::test_create(&http_addr).await;
        
  });
}


#[test]
fn test_create_with_pool() {
  let rt = Runtime::new().unwrap();
  let _ = rt.block_on(async {
        //const QUERY: &str ="SELECT name,birth_date from _test_user_ where name = ?";
    let /*mut*/ tm = TestManager::new("test_create_with_pool",3,None);
    tm.wait_for_cluster_established(1, 60).await.unwrap();
    
    let http_addr = tm.instances.get(&1).unwrap().http_addr.clone();
    crate::adapter::tests::test_create_with_pool(&http_addr).await;
        
  });
}

#[test]
fn test_adapter() {
  let rt = Runtime::new().unwrap();
  let _ = rt.block_on(async {
        //const QUERY: &str ="SELECT name,birth_date from _test_user_ where name = ?";
    let /*mut*/ tm = TestManager::new("test_adapter",3,None);
    tm.wait_for_cluster_established(1, 60).await.unwrap();
    
    let http_addr = tm.instances.get(&1).unwrap().http_addr.clone();
    crate::adapter::tests::test_adapter(&http_addr).await;
        
  });
}
