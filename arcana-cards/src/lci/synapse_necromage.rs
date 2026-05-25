//! Synapse Necromage — `{2}{B}` 3/1 black creature (Fungus Wizard).
//! "When this creature dies, create two 1/1 black Fungus creature
//! tokens with 'This token can't block.'"
//!
//! GAP: "This token can't block" — granting a static restriction
//! ability to a token is not in the TokenDefinition API. Tokens are
//! created without the can't-block ability as best effort.

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
    let name = reg.interner_mut().intern("Synapse Necromage");
    let fungus = reg.interner_mut().intern("Fungus");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fungus);
    subtypes.0.insert(wizard);
    let _ = reg.interner_mut().intern("Fungus");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: on_dies,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_dies(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let fungus_id = reg.interner().lookup("Fungus")
        .expect("Fungus interned during register()");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(fungus_id);
    let token = TokenDefinition {
        name: fungus_id,
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes: token_subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        // GAP: "can't block" restriction not expressible in TokenDefinition
        abilities: vec![],
    };
    let token2 = TokenDefinition {
        name: fungus_id,
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes: {
            let mut s = SubtypeSet::default();
            s.0.insert(fungus_id);
            s
        },
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![
        Effect::CreateToken { controller: trig.controller, token },
        Effect::CreateToken { controller: trig.controller, token: token2 },
    ]
}
