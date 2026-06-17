//! Pampered Loamfrill — `{G}` 1/1 Lizard with Deathtouch.
//!
//! Renew — `{1}{G}`, Exile this card from your graveyard: Conjure a
//! duplicate of another target creature card in your graveyard onto the
//! top of your library; the duplicate perpetually gets +1/+1 and gains
//! deathtouch. Activate only as a sorcery.
//!
//! Deathtouch is a base keyword. The Renew activated ability's payload
//! (Conjure — minting a card by name onto the library, plus a perpetual
//! buff) is an Arena-only mechanic with no `Effect` variant; the ability
//! is GAP'd but still declared as a graveyard activation so the cost is
//! recorded.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pampered Loamfrill");
    let lizard = reg.interner_mut().intern("Lizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "Renew — {1}{G}, Exile this card from your graveyard: Conjure a duplicate of another target creature card in your graveyard onto the top of your library. The duplicate perpetually gets +1/+1 and gains deathtouch. Activate only as a sorcery.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{1}{G}").expect("valid cost"),
                exile_self: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Graveyard,
            is_instant_speed: false,
            face_gate: None,
            effect: renew_conjure,
        }),
    )
}

fn renew_conjure(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Conjure not modeled (Arena-only mechanic; would need
    // registry-by-name minting + a perpetual +1/+1 / deathtouch buff).
    Vec::new()
}
