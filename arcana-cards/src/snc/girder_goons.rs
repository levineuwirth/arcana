//! Girder Goons — `{4}{B}` 4/4 Ogre Warrior.
//!
//! When this creature dies, create a tapped 2/2 black Rogue creature token.
//! Blitz {3}{B} (cast for its blitz cost: gains haste and "When this creature
//! dies, draw a card", sacrifice it at the next end step.)
//!
//! Blitz is not a usable keyword (not in the supported set) and its alternate-
//! cost mechanic is unexpressible, so it is GAP'd. The dies trigger creates a
//! 2/2 black Rogue token; the "tapped" rider is a fidelity GAP (no tapped-token
//! create primitive in the available catalog).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Girder Goons");
    let ogre = reg.interner_mut().intern("Ogre");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ogre);
    subtypes.0.insert(warrior);

    // Pre-intern the Rogue token subtype.
    let _rogue = reg.interner_mut().intern("Rogue");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![], // GAP: Blitz not a supported keyword.
        ..Default::default()
    };

    // GAP: Blitz {3}{B} alternate cost + its rider abilities — unexpressible.

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfDies,
            intervening_if: None,
            effect: dies_make_rogue,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn dies_make_rogue(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let rogue = reg.interner().lookup("Rogue").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rogue);
    // GAP (fidelity): the "tapped" rider cannot be applied — no tapped-token
    // create primitive in the available catalog.
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: rogue,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
