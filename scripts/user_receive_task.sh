echo "Test-Case 1: Student Receive Task: "
curl http://localhost:6789/receive_task\
 -H "Content-Type:application/json"\
 -d '{"userid":"wechat12307", "receive_mode":false, "target_userid":"wechat12306", "target_task":"Find Fantasy"}'
echo ""