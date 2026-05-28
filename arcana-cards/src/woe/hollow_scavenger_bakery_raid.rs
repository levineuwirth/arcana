//! Hollow Scavenger // Bakery Raid — `{2}{G}` 3/2 green Wolf.
//! "{1}, Sacrifice a Food: This creature gets +2/+2 until end of turn.
//! Activate only once each turn."
//! Adventure face "Bakery Raid" (`{G}` Sorcery): "Create a Food token."
//! GAP: "Sacrifice a Food" cost uses specific subtype — ActivationCost.sacrifice
//! is general; can't filter to Food only.
//! GAP: "Activate only once each turn" — OncePerTurn not available for activated abilities.

use arcana_core::effects::{CommodityToken, Effect};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry, SpellAbilityDef,
};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hollow Scavenger");
    let wolf = reg.interner_mut().intern("Wolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wolf);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    let adv_name = reg.interner_mut().intern("Bakery Raid");
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid adv cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "Create a Food token.".into(),
        target_requirements: Vec::new(),
        modal: None,
        effect: bakery_raid,
    };
    let adventure = CardFace {
        name: adv_name,
        characteristics: adv_chars,
        spell_ability: Some(adv_ability),
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}, Sacrifice a Food: This creature gets +2/+2 until end of turn. Activate only once each turn.".into(),
                // GAP: sacrifice a Food (specific token type) not filterable in ActivationCost
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").unwrap(),
                    sacrifice: true, // GAP: should be sacrifice-a-Food, not any creature
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_self,
            })
            .with_adventure(adventure),
    )
}

fn pump_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Pump {
        target: ctx.source,
        power: 2,
        toughness: 2,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}

fn bakery_raid(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::CreateCommodityToken {
        controller: entry.controller,
        kind: CommodityToken::Food,
        count: 1,
    }]
}
