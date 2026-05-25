//! Mona Lisa, Ever Adaptable — `{3}{G}` 4/4 legendary green Lizard Mutant. "Whenever
//! a player casts a creature spell, you create a Mutagen token. (It's an artifact…)"
//! GAP: Mutagen token is not a standard token recipe; using a plain artifact token.
//! The Mutagen's activated ability (sac + put counter) is deferred engine work.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mona Lisa, Ever Adaptable");
    let lizard = reg.interner_mut().intern("Lizard");
    let mutant = reg.interner_mut().intern("Mutant");
    let _mutagen = reg.interner_mut().intern("Mutagen");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lizard);
    subtypes.0.insert(mutant);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter::new().with_types(TypeLine::CREATURE.into())),
                    caster: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: create_mutagen_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn create_mutagen_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mutagen = reg.interner().lookup("Mutagen")
        .expect("Mutagen interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(mutagen);
    let token = TokenDefinition {
        name: mutagen,
        colors: ColorSet::colorless(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        power: None,
        toughness: None,
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: trig.controller, token }]
}
