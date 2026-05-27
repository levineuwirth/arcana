//! Station Monitor — `{W}{U}` 2/2 white/blue Lizard Artificer.
//! "Whenever you cast your second spell each turn, create a 1/1 colorless
//! Drone artifact creature token with flying and 'This token can block
//! only creatures with flying.'"
//! GAP: "second spell each turn" exact tracking not precise (approximated
//! with spells_cast_this_turn == 2); "can block only flying creatures"
//! token ability not modeled.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Station Monitor");
    let lizard = reg.interner_mut().intern("Lizard");
    let artificer = reg.interner_mut().intern("Artificer");
    let _drone = reg.interner_mut().intern("Drone");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lizard);
    subtypes.0.insert(artificer);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
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
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: second_spell_drone,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn second_spell_drone(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // Approximation: fire when spells cast this turn == 2.
    let n = script::spells_cast_this_turn(state, &ObjectFilter::new(), trig.controller);
    if n != 2 {
        return Vec::new();
    }
    let drone = reg.interner().lookup("Drone").expect("Drone interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(drone);
    let token = TokenDefinition {
        name: drone,
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        // GAP: "can block only creatures with flying" not modeled as ability
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: trig.controller, token }]
}
