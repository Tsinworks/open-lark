use serde::{Deserialize, Serialize};

use crate::feishu_card::card_components::{
    containers::column_set::ColumnSetContainer,
    content_components::{
        chart::FeishuCardChart, divider::FeishuCardDivider, image::FeishuCardImage,
        multi_image_layout::FeishuCardMultiImageLayout, note::FeishuCardNote,
        plain_text::FeishuCardText, rich_text::FeishuCardMarkdown, user_list::FeishuCardUserList,
        user_profile::FeishuCardUserProfile,
    },
    interactive_components::{
        button::FeishuCardButton, checker::Checker, date_picker::DatePicker,
        date_time_picker::DateTimePicker, input::FeishuCardInput,
        multi_select_person::MultiSelectPerson, multi_select_static::MultiSelectStatic,
        picker_time::PickerTime, select_person::SelectPerson, select_static::SelectStatic,
    },
};
use crate::feishu_card::card_components::containers::form::FormContainer;

pub mod containers;
pub mod content_components;
pub mod interactive_components;

/// 卡片组件枚举
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Element {
    ColumnSet(ColumnSetContainer),
    FormContainer(FormContainer),
    PlainText(FeishuCardText),
    Markdown(FeishuCardMarkdown),
    Image(FeishuCardImage),
    InputForm(FeishuCardInput),
    MultiImage(FeishuCardMultiImageLayout),
    Divider(FeishuCardDivider),
    UserProfile(FeishuCardUserProfile),
    UserList(FeishuCardUserList),
    Chart(FeishuCardChart),
    Note(FeishuCardNote),
    Button(FeishuCardButton),
    SelectStatic(SelectStatic),
    MultiSelect(MultiSelectStatic),
    SelectPerson(SelectPerson),
    MultiSelectPerson(MultiSelectPerson),
    DatePicker(DatePicker),
    TimeSelector(PickerTime),
    DateTimePicker(DateTimePicker),
    Checker(Checker),
}
