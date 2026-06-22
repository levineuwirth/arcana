//! Tazri, Stalwart Survivor — `{2}{W}` 3/3 Legendary Human Warrior.
//! "Each creature you control has '{T}: Add one mana of any of this
//!   creature's colors. ...'" — GAP (a static ability-granting effect, not
//!   a triggered/activated ability of Tazri; the granted any-color mana
//!   ability with its spend restriction is also unexpressible).
//! "{W}{U}{B}{R}{G}, {T}: Mill five cards. Put all creature cards with
//!   activated abilities that aren't mana abilities from among the milled
//!   cards into your hand."
//!
//! The activated ability is wired with its full five-color + tap cost. It
//! mills five. GAP: "Put all creature cards with activated abilities that
//! aren't mana abilities from among the milled cards into your hand." —
//! there is no Effect to filter just-milled cards by "has a non-mana
//! activated ability", so only the mill is performed.
//!
//! Scryfall's "Mill" keyword tag is not a KeywordAbility and does not
//! appear in the oracle text, so no keyword is emitted.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tazri, Stalwart Survivor");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: "Each creature you control has '{T}: Add one mana of any of this
    // creature's colors...'" — static ability-grant, unexpressible.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{W}{U}{B}{R}{G}, {T}: Mill five cards. Put all creature cards \
                   with activated abilities that aren't mana abilities from among \
                   the milled cards into your hand."
                .into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{W}{U}{B}{R}{G}").expect("valid cost"),
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: mill_five,
        }),
    )
}

fn mill_five(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Put all creature cards with activated abilities that aren't
    // mana abilities from among the milled cards into your hand." — no
    // Effect filters freshly-milled cards by ability shape.
    vec![Effect::Mill {
        player: ctx.controller,
        count: 5,
    }]
}
