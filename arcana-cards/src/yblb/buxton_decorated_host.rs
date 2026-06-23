//! Buxton, Decorated Host — `{3}{G}{G}{W}` 4/4 Legendary Rabbit Noble.
//! "Convoke"
//! "At the beginning of your end step, if you control a tapped creature,
//! seek a nonland permanent card with mana value X or less, where X is the
//! number of tapped creatures you control. Put that card onto the
//! battlefield."
//!
//! Convoke is not in the usable keyword surface (GAP'd). The end-step
//! trigger is wired: the "if you control a tapped creature" clause is an
//! intervening-if, and the seek-and-put-onto-battlefield is modeled with
//! `TutorToBattlefield` over a nonland-permanent filter capped at X =
//! the number of tapped creatures you control (computed at resolution).
//! Seek (random, no library search) is modeled by the closest demonstrated
//! library→battlefield primitive — a fidelity note.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::{conditions, script};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Buxton, Decorated Host");
    let rabbit = reg.interner_mut().intern("Rabbit");
    let noble = reg.interner_mut().intern("Noble");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rabbit);
    subtypes.0.insert(noble);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        // GAP: Convoke / Seek — neither is in the usable keyword surface.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // "At the beginning of your end step, if you control a tapped
            // creature, seek a nonland permanent card with mana value X or
            // less ... Put that card onto the battlefield."
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: Some(if_control_tapped_creature),
                effect: seek_nonland_permanent,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn if_control_tapped_creature(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    conditions::you_control_a(
        s,
        you,
        &ObjectFilter::creature()
            .controlled_by(ControllerConstraint::You)
            .tapped_only(),
    )
}

fn seek_nonland_permanent(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // X = the number of tapped creatures you control.
    let x = script::count_matching(
        state,
        &ObjectFilter::creature()
            .controlled_by(ControllerConstraint::You)
            .tapped_only(),
        trig.controller,
    );
    // A nonland permanent card with mana value X or less.
    let filter = ObjectFilter::permanent()
        .without_types(TypeLine::LAND.into())
        .with_max_cmc(x);
    vec![Effect::TutorToBattlefield {
        player: trig.controller,
        filter,
        tapped: false,
    }]
}
