import { UserAvatar, Card, Button } from "./components";

function testUserAvatar() {
    const avatar = UserAvatar({ user: { id: 1, name: "Test", email: "t@t.com", role: "admin" } });
    return avatar;
}

function testCard() {
    const card = Card({ title: "Test", children: null });
    return card;
}
