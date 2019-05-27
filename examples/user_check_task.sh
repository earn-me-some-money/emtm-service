echo "Test-Case 1: Cow Check Task: "
curl http://localhost:8088/check_task\
 -H "Content-Type:application/json"\
 -d '{"userid":"wechat12306", "check_mode":false, "task_name":"Find Fantasy"}'
echo ""

echo "Test-Case 2: Student Check Task: "
curl http://localhost:8088/check_task\
 -H "Content-Type:application/json"\
 -d '{"userid":"wechat12307", "check_mode":true, "task_name":"Pick Me Up"}'
echo ""