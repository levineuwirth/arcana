//! Bloodrage Alpha — `{3}{R}` 4/4 Creature — Wolf.
//!
//! * When this creature enters the battlefield, choose one —
//!   • Another target Wolf or Werewolf you control fights target creature you
//!     don't control.
//!   • You get a one-time boon with "When you cast a Wolf or Werewolf spell, it
//!     gains 'When this creature enters, it fights up to one target creature you
//!     don't control.'"
//!
//! Modal TRIGGERED abilities aren't supported (the demonstrated modal surface
//! is for spell abilities only). The ETB trigger is wired to the first mode
//! (the Fight), targeting another Wolf/Werewolf you control and a creature you
//! don't control.
//! GAP: the modal choice itself is not expressible on a trigger.
//! GAP: mode 2 (a one-time boon granting a cast-rider) has no representation.
//! (Scryfall keyword "Fight" is an ability word, not a `KeywordAbility`.)

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Bloodrage Alpha");
    let wolf = reg.interner_mut().intern("Wolf");
    let werewolf = reg.interner_mut().intern("Werewolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wolf);

    // Subtype-OR filter for "Wolf or Werewolf you control", built in register
    // (needs the interner).
    let wolf_or_werewolf = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_subtypes_any(vec![wolf, werewolf]);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_fight,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Permanent(wolf_or_werewolf),
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::creature()
                                .controlled_by(ControllerConstraint::Opponent),
                        ),
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                ],
            }),
    )
}

fn etb_fight(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut ids = trig.targets.targets.iter().filter_map(|t| match t {
        TargetChoice::Object(id) => Some(*id),
        _ => None,
    });
    let (Some(a), Some(b)) = (ids.next(), ids.next()) else {
        return Vec::new();
    };
    vec![Effect::Fight { a, b }]
}
