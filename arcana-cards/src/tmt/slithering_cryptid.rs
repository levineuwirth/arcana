//! Slithering Cryptid — `{2}{G/U}` 2/3 green/blue Fish Mutant.
//! "When this creature enters, create a Mutagen token."
//!
//! GAP: Mutagen token type is not in the commodity token catalog. Using a generic
//! artifact token as the closest available approximation — no activated ability.

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
    let name = reg.interner_mut().intern("Slithering Cryptid");
    let fish = reg.interner_mut().intern("Fish");
    let mutant = reg.interner_mut().intern("Mutant");
    let _mutagen = reg.interner_mut().intern("Mutagen");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fish);
    subtypes.0.insert(mutant);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G/U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: on_etb,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_etb(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mutagen = reg.interner().lookup("Mutagen").expect("Mutagen interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(mutagen);
    // GAP: Mutagen activated ability not expressible in TokenDefinition.
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
