//! Lulu, Loyal Hollyphant — `{3}{W}` 3/2 Legendary Elephant Angel.
//! Flying.
//! "At the beginning of your end step, if a permanent you controlled
//! left the battlefield this turn, put a +1/+1 counter on each tapped
//! creature you control, then untap them."
//! "Choose a Background" (GAP — not a usable KeywordAbility).
//!
//! Flying is a whitelisted unit keyword. The end-step trigger's
//! intervening-if ("a permanent you controlled left the battlefield this
//! turn") has no matching condition helper — GAP'd, so the trigger fires
//! every end step (the counter/untap body is faithful).

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lulu, Loyal Hollyphant");
    let elephant = reg.interner_mut().intern("Elephant");
    let angel = reg.interner_mut().intern("Angel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elephant);
    subtypes.0.insert(angel);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "Choose a Background" — not a usable KeywordAbility variant.

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::End,
                whose: ControllerConstraint::You,
            },
            // GAP (intervening-if): "if a permanent you controlled left the
            // battlefield this turn" has no matching condition helper.
            intervening_if: None,
            effect: counter_and_untap_tapped,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn counter_and_untap_tapped(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .tapped_only();
    let ids = script::ids_matching(state, &filter, trig.controller);
    let mut effects: Vec<Effect> = Vec::new();
    for id in &ids {
        effects.push(Effect::AddCounters {
            target: *id,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        });
    }
    for id in &ids {
        effects.push(Effect::Untap { target: *id });
    }
    vec![Effect::Sequence(effects)]
}
