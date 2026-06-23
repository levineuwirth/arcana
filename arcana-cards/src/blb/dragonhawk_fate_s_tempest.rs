//! Dragonhawk, Fate's Tempest — `{3}{R}{R}` legendary 5/5 Bird Dragon
//! with Flying.
//!
//! Oracle:
//! * Flying — keyword.
//! * "Whenever Dragonhawk enters or attacks, exile the top X cards of
//!   your library, where X is the number of creatures you control with
//!   power 4 or greater. You may play those cards until your next end
//!   step. At the beginning of your next end step, Dragonhawk deals 2
//!   damage to each opponent for each of those cards that are still
//!   exiled."
//!   - "enters or attacks" is two trigger conditions, so it is wired as
//!     two `TriggeredAbilityDef`s (SelfEntersBattlefield + SelfAttacks)
//!     with the same resolver.
//!   - The "exile top X, you may play them until end of turn" body is
//!     `Effect::ImpulseExile`, with X computed dynamically at
//!     resolution (creatures you control with power >= 4).
//!   - GAP: the delayed "deal 2 damage to each opponent for each of
//!     those cards still exiled at your next end step" — there is no
//!     primitive to schedule a delayed amount keyed on how many of THIS
//!     impulse's cards remain exiled.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dragonhawk, Fate's Tempest");
    let bird = reg.interner_mut().intern("Bird");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: impulse_for_x,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: impulse_for_x,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn impulse_for_x(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // X = number of creatures you control with power 4 or greater.
    let x = script::count_matching(
        state,
        &ObjectFilter::creature()
            .controlled_by(ControllerConstraint::You)
            .with_min_power(4),
        trig.controller,
    );
    if x == 0 {
        return Vec::new();
    }
    // GAP: the delayed "at your next end step, deal 2 damage to each
    // opponent for each of those cards still exiled" — no primitive
    // tracks the leftover of this specific impulse exile.
    vec![Effect::ImpulseExile { player: trig.controller, count: x }]
}
