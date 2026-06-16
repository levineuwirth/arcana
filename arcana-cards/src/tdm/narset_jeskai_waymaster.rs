//! Narset, Jeskai Waymaster — `{U}{R}{W}` 3/4 Legendary blue/red/white
//! Human Monk. "At the beginning of your end step, you may discard your
//! hand. If you do, draw cards equal to the number of spells you've
//! cast this turn."

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::targets::ControllerConstraint;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Narset, Jeskai Waymaster");
    let human = reg.interner_mut().intern("Human");
    let monk = reg.interner_mut().intern("Monk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(monk);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{R}{W}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: discard_hand_draw_spells,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn discard_hand_draw_spells(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let n = script::spells_cast_this_turn(
        state,
        &ObjectFilter::new(),
        trig.controller,
    );
    // "you may discard your hand. If you do, draw N cards"
    // Discard hand = discard all cards. hand_size gives current count.
    let hand = script::hand_size(state, trig.controller);
    
    // GAP: OptionalPayment only supports Mana/Life costs, not "discard hand".
    // Emitting sequence as best effort: discard hand then draw.
    if hand == 0 {
        return Vec::new();
    }
    vec![Effect::Sequence(vec![
        Effect::Discard {
            player: trig.controller,
            count: hand,
            choice: DiscardChoice::ControllerChooses,
        },
        Effect::DrawCards { player: trig.controller, count: n },
    ])]
}
