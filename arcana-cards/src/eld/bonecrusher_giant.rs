//! Bonecrusher Giant // Stomp — `{1}{R}` Giant creature 4/3 with the
//! Adventure face "Stomp" (`{1}{R}` instant, "Stomp deals 2 damage to
//! any target"). Throne of Eldraine uncommon. The seed's touchstone
//! for CR 715 Adventurer.
//!
//! # Rules references
//!
//! * CR 715.1 — Adventurer. The card has a creature main face and an
//!   Adventure face (instant or sorcery); the player may cast either,
//!   and casting the Adventure face uses its own name, mana cost,
//!   type line, and rules text per CR 715.2.
//! * CR 715.4 — If the Adventure spell leaves the stack (resolution,
//!   counter, fizzle), it's exiled rather than going to the
//!   graveyard. While exiled, the card's owner may cast it as a
//!   creature spell for its creature-face cost.
//! * CR 715.5 — Once the card leaves exile (cast as creature, moved
//!   by another effect, cleanup), it reverts to being a normal
//!   creature card with no lingering adventure markers. The engine
//!   side of that is the zone-change re-id (CR 400.7) dropping
//!   [`GameObject::adventure_exile_pending`].
//!
//! # Engine wiring
//!
//! The creature half rides on
//! [`CardDefinition::base_characteristics`] and
//! [`CardDefinition::spell_ability`]; the Adventure half lives on
//! [`CardDefinition::alternate_face`] as an
//! [`AlternateFace::Adventure(CardFace)`]. The cast pipeline routes
//! an Adventure cast to the face's spell ability (see
//! [`arcana_core::engine::resolution_effects`]); the finalize /
//! counter paths route the card to exile with the flag set (see
//! [`StackEntry::pre_adventure_characteristics`] +
//! `finalize_resolved_spell`).

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    CardDefinition, CardFace, CardRegistry, SpellAbilityDef,
};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectOrPlayer, TargetChoice, TargetRequirement};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bonecrusher Giant");
    let giant_sub = reg.interner_mut().intern("Giant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(giant_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid creature cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // Adventure face "Stomp" — instant `{1}{R}`, 2 damage to any
    // target. The face name is separate from the card name (CR
    // 715.2).
    let stomp_name = reg.interner_mut().intern("Stomp");
    let stomp_chars = Characteristics {
        name: stomp_name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid stomp cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    let stomp_ability = SpellAbilityDef {
        text: "Stomp deals 2 damage to any target.".into(),
        target_requirements: vec![TargetRequirement::any_target()],
        modal: None,
        effect: stomp_resolve,
    };
    let adventure = CardFace {
        name: stomp_name,
        characteristics: stomp_chars,
        spell_ability: Some(stomp_ability),
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_adventure(adventure),
    )
}

fn stomp_resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let dt = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
    };
    vec![Effect::DealDamage {
        source: entry.source,
        target: dt,
        amount: 2,
    }]
}

