echo "Test-Case 1: Cow-User Login: "
curl http://localhost:6789/login\
 -H "Content-Type:application/json"\
 -d '{"userid":"wechat12306", "wechat_ok":true, "login_mode":false}'
echo " "

echo "Test-Case 2: Student-User Login: "
curl http://localhost:6789/login\
 -H "Content-Type:application/json"\
 -d '{"userid":"wechat12307", "wechat_ok":true, "login_mode":true}'
echo " "
