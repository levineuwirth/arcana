//! A Girl and Her Dogs — `{4}{W}` 3/3 white Creature — Human.
//! "When A Girl and Her Dogs enters the battlefield and at the beginning of
//! your upkeep, create a 1/1 white legendary Dog creature token and name it."
//! (the "and name it" flavor naming is cosmetic and unmodeled)
//! "Whenever A Girl and Her Dogs attacks, it gets +1/+1 until end of turn for
//! each legendary creature you control."

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::layers::Duration;
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
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("A Girl and Her Dogs");
    let human = reg.interner_mut().intern("Human");
    let _dog = reg.interner_mut().intern("Dog");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            // "When ~ enters … create a Dog token"
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: make_dog,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // "… and at the beginning of your upkeep, create a Dog token"
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: make_dog,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // "Whenever ~ attacks, it gets +1/+1 … for each legendary creature you control"
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: pump_for_legendaries,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn make_dog(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let dog = reg.interner().lookup("Dog").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dog);
    let token = TokenDefinition {
        name: dog,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    // GAP: "name it" — bestowing a name on a token is purely cosmetic and not
    // modeled. Token is created as a 1/1 white legendary Dog (legendary
    // supertype on a token is not expressible via TokenDefinition).
    vec![Effect::CreateToken {
        controller: trig.controller,
        token,
    }]
}

fn pump_for_legendaries(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_supertypes(SupertypeSet::new().with(SupertypeSet::LEGENDARY));
    let n = script::count_matching(state, &filter, trig.controller) as i32;
    vec![Effect::Pump {
        target: trig.source,
        power: n,
        toughness: n,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
