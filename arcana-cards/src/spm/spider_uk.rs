//! Spider-UK — `{3}{W}` 3/4 Legendary Creature — Spider Human Hero.
//!
//! Oracle:
//! * Web-slinging {2}{W} — an alternative cost (cast for {2}{W} if you also
//!   return a tapped creature you control to its owner's hand). The alt-cast
//!   mechanic is NOT in the usable keyword/cost surface; GAP'd.
//! * "At the beginning of your end step, if two or more creatures entered the
//!   battlefield under your control this turn, you draw a card and gain 2
//!   life." — an end-step trigger gated by an intervening-if clause.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

// GAP: Web-slinging {2}{W} — alternative cast cost ("return a tapped creature
// you control to its owner's hand") is not expressible with the usable cost
// surface.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Spider-UK");
    let spider = reg.interner_mut().intern("Spider");
    let human = reg.interner_mut().intern("Human");
    let hero = reg.interner_mut().intern("Hero");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spider);
    subtypes.0.insert(human);
    subtypes.0.insert(hero);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::End,
                whose: ControllerConstraint::You,
            },
            intervening_if: Some(if_two_creatures_entered),
            effect: draw_and_gain,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn if_two_creatures_entered(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    // "if two or more creatures entered the battlefield under your control
    // this turn"
    script::entered_this_turn_matching(
        s,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        you,
    ) >= 2
}

fn draw_and_gain(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![
        Effect::DrawCards {
            player: trig.controller,
            count: 1,
        },
        Effect::GainLife {
            player: trig.controller,
            amount: 2,
        },
    ]
}
