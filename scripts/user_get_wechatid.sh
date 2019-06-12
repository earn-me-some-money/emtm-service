echo "Test-Case 1: Get User wechat-id"
curl http://localhost:6789/get_wechatid\
 -H "Content-Type:application/json"\
 -d '{"appid":"wx9d86195b9f2c0137", "secret":"6260719c0a702f13c1698ca47beb60bc", "code":"023VmiSj22JeyE06vZQj2Gr7Sj2VmiSt"}'
echo ""
