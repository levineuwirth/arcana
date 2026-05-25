//! Great Desert Prospector — `{4}{W}` 3/2 white Human Artificer. "When this creature
//! enters, create a tapped Powerstone token for each other creature you control."

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
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Great Desert Prospector");
    let human = reg.interner_mut().intern("Human");
    let artificer = reg.interner_mut().intern("Artificer");
    let _powerstone = reg.interner_mut().intern("Powerstone");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(artificer);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_create_powerstones,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_create_powerstones(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let count = script::count_matching(
        state,
        &ObjectFilter::creature()
            .controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    // exclude self (just entered)
    let n = if count > 0 { count - 1 } else { 0 };
    let powerstone = reg.interner().lookup("Powerstone")
        .expect("Powerstone interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(powerstone);
    let token = TokenDefinition {
        name: powerstone,
        colors: ColorSet::colorless(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        power: None,
        toughness: None,
        keywords: vec![],
        abilities: vec![],
    };
    // GAP: token enters tapped — CreateToken does not model tapped entry
    (0..n)
        .map(|_| Effect::CreateToken { controller: trig.controller, token: token.clone() })
        .collect()
}
