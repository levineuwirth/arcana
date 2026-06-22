//! Szarekh, the Silent King — `{1}{B}{B}{B}` 3/4 Legendary Artifact Creature —
//! Necron, with Flying.
//!
//! Flying
//! My Will Be Done — Whenever Szarekh attacks, mill three cards. You may put an
//! artifact creature card or Vehicle card from among the cards milled this way
//! into your hand.
//!
//! "My Will Be Done" and "Mill" are ability-word / reminder labels, not real
//! KeywordAbility variants — only Flying is a base keyword.
//! The attack trigger mills three cards (expressible). The follow-up "you may
//! put an artifact creature/Vehicle card from among the cards milled this way
//! into your hand" is a selection scoped to *just the cards milled this way* with
//! no Effect primitive for that scoped pick — GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Szarekh, the Silent King");
    let necron = reg.interner_mut().intern("Necron");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(necron);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_mill,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn attack_mill(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "you may put an artifact creature or Vehicle card from among the cards
    // milled this way into your hand" — no scoped-to-milled-cards pick primitive.
    vec![Effect::Mill { player: trig.controller, count: 3 }]
}
