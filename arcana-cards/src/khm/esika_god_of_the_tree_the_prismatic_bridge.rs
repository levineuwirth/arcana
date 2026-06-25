//! Esika, God of the Tree // The Prismatic Bridge
//!
//! Front: Legendary Creature — God {1}{G}{G} 1/4 (green)
//!   Vigilance; {T}: Add one mana of any color.
//!   Other legendary creatures you control have vigilance and "{T}: Add one mana of any color."
//! Back: Legendary Enchantment
//!   At the beginning of your upkeep, reveal cards from the top of your library until you reveal a creature or planeswalker card. Put that card onto the battlefield.
//! Esika's own "{T}: Add one mana of any color" is modeled as five mana
//! abilities, one per WUBRG color (command_tower idiom), face-gated to the
//! front face; the shared {T} cost means activating one taps the source, so
//! only one fires.
//! GAP: MDFC back face upkeep trigger not modeled (mechanic deferred)
//! GAP: Static ability granting vigilance + "{T}: Add one mana of any color"
//!      to other legendary creatures you control not modeled

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Esika, God of the Tree");
    let back_name = reg.interner_mut().intern("The Prismatic Bridge");
    let god = reg.interner_mut().intern("God");

    let mut subtypes = SubtypeSet::new();
    subtypes.insert(god);

    let chars = Characteristics {
        mana_cost: Some(ManaCost::parse("{1}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    let back_chars = Characteristics {
        mana_cost: Some(ManaCost::parse("{1}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_mdfc_back(CardFace {
                name: back_name,
                characteristics: back_chars,
                spell_ability: None,
            })
            .with_activated_ability(mana_ability("{T}: Add {W}.", add_white_mana))
            .with_activated_ability(mana_ability("{T}: Add {U}.", add_blue_mana))
            .with_activated_ability(mana_ability("{T}: Add {B}.", add_black_mana))
            .with_activated_ability(mana_ability("{T}: Add {R}.", add_red_mana))
            .with_activated_ability(mana_ability("{T}: Add {G}.", add_green_mana)),
    )
}

fn mana_ability(
    text: &str,
    effect: fn(&GameState, &ActivationContext, &CardRegistry) -> Vec<Effect>,
) -> ActivatedAbilityDef {
    ActivatedAbilityDef {
        text: text.into(),
        cost: ActivationCost::tap_only(),
        target_requirements: Vec::new(),
        is_mana_ability: true,
        is_loyalty_ability: false,
        activation_zone: ActivationZone::Battlefield,
        is_instant_speed: false,
        face_gate: Some(0),
        effect,
    }
}

fn add_white_mana(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::White, ctx.source)],
    }]
}

fn add_blue_mana(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Blue, ctx.source)],
    }]
}

fn add_black_mana(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Black, ctx.source)],
    }]
}

fn add_red_mana(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Red, ctx.source)],
    }]
}

fn add_green_mana(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Green, ctx.source)],
    }]
}
