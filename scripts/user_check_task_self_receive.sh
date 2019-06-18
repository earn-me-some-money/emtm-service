echo "Test-Case 1: Cow Check Task Self Receive: "
curl http://localhost:6789/check_task_self_receive\
 -H "Content-Type:application/json"\
 -d '{"userid":"wechat12307"}'
echo ""

echo "Test-Case 2: Student Check Task Self Receive: "
curl http://localhost:6789/check_task_self_receive\
 -H "Content-Type:application/json"\
 -d '{"userid":"wechat12306"}'
echo ""
