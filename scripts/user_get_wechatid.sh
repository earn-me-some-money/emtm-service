echo "Test-Case 1: Get User wechat-id"
curl http://localhost:6789/get_wechatid\
 -H "Content-Type:application/json"\
 -d '{"appid":"wx1d993d127febb116", "secret":"291d7ab3aca41cf19aab31d7357ed9d1", "code":""}'
echo ""