//! Spara's Adjudicators — `{2}{G}{W}{U}` 4/4 Cat Citizen.
//!
//! Oracle:
//!  * When this creature enters, target creature an opponent controls can't
//!    attack or block until your next turn.
//!  * {2}, Exile this card from your hand: Target land gains "{T}: Add {G},
//!    {W}, or {U}" until this card is cast from exile. You may cast this card
//!    for as long as it remains exiled. (GAP — see below.)

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Spara's Adjudicators");
    let cat = reg.interner_mut().intern("Cat");
    let citizen = reg.interner_mut().intern("Citizen");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    subtypes.0.insert(citizen);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{W}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: "{2}, Exile this card from your hand: Target land gains a mana
    // ability until this card is cast from exile; you may cast this card while
    // exiled" is an exile-from-hand activated ability that grants a temporary
    // ability with a cast-from-exile-keyed duration — not expressible with the
    // demonstrated ActivationCost / Effect API.

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_lock_down,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
        }),
    )
}

fn etb_lock_down(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![
        Effect::ForbidAttacking {
            target: *id,
            duration: Duration::UntilYourNextTurn(trig.controller),
        },
        Effect::ForbidBlocking {
            target: *id,
            duration: Duration::UntilYourNextTurn(trig.controller),
        },
    ]
}
