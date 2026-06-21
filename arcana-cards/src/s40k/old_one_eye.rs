//! Old One Eye — `{5}{G}` 6/6 Legendary Creature — Tyranid with
//! Trample.
//! "Other creatures you control have trample.
//!  When Old One Eye enters, create a 5/5 green Tyranid creature token.
//!  Fast Healing — At the beginning of your first main phase, you may
//!  discard two cards. If you do, return this card from your graveyard
//!  to your hand."
//!
//! GAP: "Fast Healing" is an ability word, not a supported keyword.
//! GAP: "Other creatures you control have trample" is a static
//! continuous ability — not a triggered/activated ability.
//! GAP: the Fast Healing trigger's "you may discard two cards. If you
//! do, …" is an optional-discard cost gate; OptionalPaymentKind only
//! supports Mana/Life, so the gate (and thus the graveyard return) is
//! not expressible.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Old One Eye");
    let tyranid = reg.interner_mut().intern("Tyranid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tyranid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: make_tyranid_token,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn make_tyranid_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let tyranid = reg.interner().lookup("Tyranid").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tyranid);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: tyranid,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(5)),
            toughness: Some(PtValue::Fixed(5)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
