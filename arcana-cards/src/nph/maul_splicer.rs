//! Maul Splicer — `{6}{G}` 1/1 Phyrexian Human Artificer.
//! "When this creature enters, create two 3/3 colorless Phyrexian Golem
//!  artifact creature tokens.
//!  Golem creatures you control have trample."
//!
//! The ETB two-Golem-token trigger is wired. The static "Golem creatures
//! you control have trample" is a continuous static anthem and is GAP'd.

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

// GAP: static — "Golem creatures you control have trample" is a continuous
// static keyword-granting anthem (no trigger word, no activation cost).

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Maul Splicer");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let human = reg.interner_mut().intern("Human");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(human);
    subtypes.0.insert(artificer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_make_golems,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_make_golems(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let phyrexian = reg.interner().lookup("Phyrexian").unwrap_or_default();
    let golem = reg.interner().lookup("Golem").unwrap_or_default();
    let make = || {
        let mut subtypes = SubtypeSet::default();
        subtypes.0.insert(phyrexian);
        subtypes.0.insert(golem);
        Effect::CreateToken {
            controller: trig.controller,
            token: TokenDefinition {
                name: golem,
                colors: ColorSet::colorless(),
                types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
                subtypes,
                power: Some(PtValue::Fixed(3)),
                toughness: Some(PtValue::Fixed(3)),
                keywords: vec![],
                abilities: vec![],
            },
        }
    };
    vec![make(), make()]
}
