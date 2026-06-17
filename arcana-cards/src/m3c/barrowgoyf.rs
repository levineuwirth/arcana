//! Barrowgoyf — `{2}{B}` Lhurgoyf with Deathtouch and Lifelink. Power/toughness are
//! a characteristic-defining `*/1+*` (count of card types among cards in all
//! graveyards / that number + 1) — not expressible as a Fixed P/T, so left to default
//! (GAP: CDA P/T). "Whenever this creature deals combat damage to a player, you may
//! mill that many cards. If you do, you may put a creature card from among them into
//! your hand." We mill the damage amount; the "put a creature card from among them"
//! rider is GAP'd (no among-milled-into-hand primitive).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Barrowgoyf");
    let lhurgoyf = reg.interner_mut().intern("Lhurgoyf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lhurgoyf);

    // GAP: characteristic-defining */1+* power/toughness — left as default.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        keywords: vec![KeywordAbility::Deathtouch, KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter::new(),
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: mill_that_many,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn mill_that_many(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let n = trig.damage_amount().unwrap_or(0);
    // GAP: "you may put a creature card from among them into your hand" — no
    // among-milled-selection primitive.
    vec![Effect::Mill { player: trig.controller, count: n }]
}
