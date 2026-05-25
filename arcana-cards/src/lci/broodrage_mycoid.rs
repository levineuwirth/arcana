//! Broodrage Mycoid — `{3}{B}` 4/3 black Fungus.
//! "At the beginning of your end step, if you descended this turn, create a 1/1 black
//! Fungus creature token with 'This token can't block.'"
//!
//! GAP: "descended this turn" (a permanent card was put into your graveyard from anywhere)
//! is not expressible as an intervening-if. Emitting unconditionally. Also "can't block"
//! on a token is not expressible in TokenDefinition.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Broodrage Mycoid");
    let fungus = reg.interner_mut().intern("Fungus");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fungus);
    let _fungus_tok = reg.interner_mut().intern("Fungus");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
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
                // GAP: intervening-if "if you descended this turn" — use None
                intervening_if: None,
                effect: on_end_step,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_end_step(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let fungus = reg.interner().lookup("Fungus").expect("Fungus interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fungus);
    // GAP: "can't block" ability on token not expressible in TokenDefinition.
    let token = TokenDefinition {
        name: fungus,
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: trig.controller, token }]
}
