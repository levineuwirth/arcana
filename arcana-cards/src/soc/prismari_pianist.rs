//! Prismari Pianist — `{1}{R}{R}` 2/1 red Djinn Bard.
//! "Whenever you cast an instant or sorcery spell, create a 1/1 blue and
//! red Elemental creature token. If that spell's mana value is 5 or
//! greater, create three of those tokens instead."
//!
//! GAP: effect — "if that spell's mana value is 5 or greater, create three
//! instead" conditional on the triggering spell's CMC is not computable
//! (no accessor for triggering spell's CMC). Creating one token unconditionally.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Prismari Pianist");
    let djinn = reg.interner_mut().intern("Djinn");
    let bard = reg.interner_mut().intern("Bard");
    let _elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(djinn);
    subtypes.0.insert(bard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter {
                        types_any: Some(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY)),
                        ..Default::default()
                    }),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: create_elemental_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn create_elemental_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let elemental = reg.interner().lookup("Elemental")
        .expect("Elemental interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    let token = TokenDefinition {
        name: elemental,
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    // GAP: effect — "if MV >= 5 create three instead" not computable; creating one.
    vec![Effect::CreateToken { controller: trig.controller, token }]
}
