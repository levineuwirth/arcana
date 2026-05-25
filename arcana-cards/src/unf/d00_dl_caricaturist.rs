//! D00-DL, Caricaturist — `{6}` 1/1 colorless Legendary Artifact Creature — Robot.
//! "When D00-DL, Caricaturist enters, create a 4/4 colorless Sketch creature token,
//! which you have fifteen seconds to draw. The token has flying if it has wings in
//! its art. ..."
//! GAP: real-world drawing mechanic and art-conditional keywords not expressible;
//! creating a plain 4/4 colorless Sketch token as approximation.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("D00-DL, Caricaturist");
    let robot = reg.interner_mut().intern("Robot");
    let _sketch = reg.interner_mut().intern("Sketch");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(robot);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: create_sketch_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn create_sketch_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let sketch = reg.interner().lookup("Sketch")
        .expect("Sketch interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sketch);
    let token = TokenDefinition {
        name: sketch,
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        // GAP: art-conditional keywords not expressible
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: trig.controller, token }]
}
