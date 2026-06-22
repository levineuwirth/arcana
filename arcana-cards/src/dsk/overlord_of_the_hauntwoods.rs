//! Overlord of the Hauntwoods — `{3}{G}{G}` 6/5 Enchantment Creature — Avatar
//! Horror. Impending 4.
//! "Whenever this permanent enters or attacks, create a tapped colorless land
//! token named Everywhere that is every basic land type."
//!
//! Decomposition:
//! * Impending 4 — alternative-cast keyword not in supported surface; GAP'd.
//! * "Enters or attacks → make Everywhere land" — split into two triggers
//!   (`SelfEntersBattlefield` + `SelfAttacks`), each creating a colorless land
//!   token with all five basic land subtypes named "Everywhere". ("tapped" is a
//!   fidelity gap; CreateToken has no tapped option.)

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
    let name = reg.interner_mut().intern("Overlord of the Hauntwoods");
    let avatar = reg.interner_mut().intern("Avatar");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(avatar);
    subtypes.0.insert(horror);
    // Pre-intern token name + basic land subtypes for the resolver.
    let _everywhere = reg.interner_mut().intern("Everywhere");
    let _plains = reg.interner_mut().intern("Plains");
    let _island = reg.interner_mut().intern("Island");
    let _swamp = reg.interner_mut().intern("Swamp");
    let _mountain = reg.interner_mut().intern("Mountain");
    let _forest = reg.interner_mut().intern("Forest");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    // GAP: Impending 4—{1}{G}{G} — alternative-cast keyword not supported.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: make_everywhere,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: make_everywhere,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn make_everywhere(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let everywhere = reg.interner().lookup("Everywhere").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    for s in ["Plains", "Island", "Swamp", "Mountain", "Forest"] {
        if let Some(sym) = reg.interner().lookup(s) {
            subtypes.0.insert(sym);
        }
    }
    // GAP: token enters tapped — CreateToken has no tapped option.
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: everywhere,
            colors: ColorSet::colorless(),
            types: TypeLine::LAND.into(),
            subtypes,
            power: None,
            toughness: None,
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
