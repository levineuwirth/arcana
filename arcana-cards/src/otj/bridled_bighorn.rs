//! Bridled Bighorn — `{3}{W}` 3/4 Sheep Mount with Vigilance.
//!
//! Oracle:
//! * Vigilance
//! * Whenever this creature attacks while saddled, create a 1/1 white Sheep
//!   creature token.
//! * Saddle 2
//!
//! Vigilance is a base keyword. Saddle is not a usable keyword (no Saddle
//! variant, and the saddle activated ability is not expressible). The attack
//! trigger fires on `SelfAttacks`; the "while saddled" gate has no expressible
//! state accessor (GAP'd — the token is created on every attack, not only when
//! saddled). The token payload (a 1/1 white Sheep) is expressible.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
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
    let name = reg.interner_mut().intern("Bridled Bighorn");
    let sheep = reg.interner_mut().intern("Sheep");
    let mount = reg.interner_mut().intern("Mount");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sheep);
    subtypes.0.insert(mount);

    // Pre-intern the token's subtype.
    let _sheep_tok = reg.interner_mut().intern("Sheep");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    // GAP: "Saddle 2" — Saddle is not a usable keyword / activated ability.

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: make_sheep,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn make_sheep(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "while saddled" gate has no expressible accessor — fires on every
    // attack.
    let sheep = reg.interner().lookup("Sheep").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sheep);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: sheep,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
