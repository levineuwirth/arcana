//! Minion of the Mighty — `{R}` 0/1 Creature — Kobold.
//!
//! Oracle:
//! * Menace.
//! * Pack tactics — Whenever this creature attacks, if you attacked with
//!   creatures with total power 6 or greater this combat, you may put a
//!   Dragon creature card from your hand onto the battlefield tapped and
//!   attacking.
//!   GAP (intervening-if): no condition helper for "total attacking power
//!   6 or greater this combat" — the put fires unconditionally on attack.
//!   The "Pack tactics" keyword itself is not on the supported surface.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Minion of the Mighty");
    let kobold = reg.interner_mut().intern("Kobold");
    let _dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kobold);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: on_attack_put_dragon,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn on_attack_put_dragon(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = script::subtype_filter(reg, "Dragon").with_types(TypeLine::CREATURE.into());
    vec![Effect::PutFromHandOntoBattlefieldTappedAttacking {
        player: trig.controller,
        filter,
    }]
}
