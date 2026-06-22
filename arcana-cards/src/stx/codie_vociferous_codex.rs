//! Codie, Vociferous Codex — `{3}` 1/4 Legendary Artifact Creature — Book Construct.
//! "You can't cast permanent spells.
//!  {4}, {T}: Add {W}{U}{B}{R}{G}. When you next cast a spell this turn, exile cards
//!  from the top of your library until you exile an instant or sorcery card with
//!  lesser mana value. Until end of turn, you may cast that card without paying its
//!  mana cost. Put each other card exiled this way on the bottom of your library in a
//!  random order."
//!
//! The "can't cast permanent spells" static is GAP'd (no casting-restriction static).
//! The activated ability adds {W}{U}{B}{R}{G}; its reflexive next-cast impulse rider
//! is GAP'd. Because the ability carries that rider it is not a pure mana ability.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Codie, Vociferous Codex");
    let book = reg.interner_mut().intern("Book");
    let construct = reg.interner_mut().intern("Construct");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(book);
    subtypes.0.insert(construct);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: static "You can't cast permanent spells." — a spell-casting restriction
    // static is not expressible.

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{4}, {T}: Add {W}{U}{B}{R}{G}. When you next cast a spell this turn, exile cards from the top of your library until you exile an instant or sorcery card with lesser mana value. Until end of turn, you may cast that card without paying its mana cost. Put each other card exiled this way on the bottom of your library in a random order.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: tap_for_wubrg,
            }),
    )
}

fn tap_for_wubrg(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: the reflexive "When you next cast a spell this turn, exile … cast that card
    // for free …" rider is a next-cast cascade-like impulse keyed on mana value lower
    // than the cast spell — not expressible. Emit the mana add faithfully.
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![
            ManaUnit::plain(ManaColor::White, ctx.source),
            ManaUnit::plain(ManaColor::Blue, ctx.source),
            ManaUnit::plain(ManaColor::Black, ctx.source),
            ManaUnit::plain(ManaColor::Red, ctx.source),
            ManaUnit::plain(ManaColor::Green, ctx.source),
        ],
    }]
}
