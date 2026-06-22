//! Goblin Chainwhirler — `{R}{R}{R}` 3/3 red Goblin Warrior with First
//! strike.
//!
//! Oracle:
//! * First strike.
//! * When this creature enters, it deals 1 damage to each opponent and
//!   each creature and planeswalker they control.
//!
//! The ETB sweep deals 1 to every opponent player and 1 to each creature
//! or planeswalker an opponent controls. The damage source is this
//! creature.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Goblin Chainwhirler");
    let goblin = reg.interner_mut().intern("Goblin");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::FirstStrike],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_chainwhirl,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

/// Deal 1 damage to each opponent and to each creature/planeswalker they
/// control.
fn etb_chainwhirl(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = Vec::new();

    for p in script::opponents(state, trig.controller) {
        effects.push(Effect::DealDamage {
            source: trig.source,
            target: DamageTarget::Player(p),
            amount: 1,
        });
    }

    let filter = ObjectFilter::permanent()
        .with_types_any(TypeLine(TypeLine::CREATURE | TypeLine::PLANESWALKER))
        .controlled_by(ControllerConstraint::Opponent);
    for id in script::ids_matching(state, &filter, trig.controller) {
        effects.push(Effect::DealDamage {
            source: trig.source,
            target: DamageTarget::Object(id),
            amount: 1,
        });
    }

    effects
}
