//! Biomathematician — `{1}{G}{U}` 2/2 green-blue Human Wizard.
//! "When this creature enters, create a 0/0 green and blue Fractal creature token.
//! Put a +1/+1 counter on each Fractal you control."

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Biomathematician");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let _fractal = reg.interner_mut().intern("Fractal");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_fractal,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_fractal(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let fractal = reg.interner().lookup("Fractal")
        .expect("Fractal interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fractal);
    let token = TokenDefinition {
        name: fractal,
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![],
        abilities: vec![],
    };
    let fractal_ids = script::ids_matching(
        state,
        &script::subtype_filter(reg, "Fractal"),
        trig.controller,
    );
    let counter_each = Effect::ForEach {
        targets: fractal_ids,
        effect: Box::new(Effect::AddCounters {
            target: NULL_OBJECT_ID,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        }),
    };
    vec![
        Effect::CreateToken { controller: trig.controller, token },
        counter_each,
    ]
}
