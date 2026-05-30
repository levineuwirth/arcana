//! Sun-Blessed Guardian // Furnace-Blessed Conqueror — `{1}{W}` Creature —
//! Human Cleric 2/2 (front).
//! Front: `{5}{R/P}: Transform this creature. Activate only as a sorcery.`
//! ({R/P} can be paid with {R} or 2 life.)
//! Back: Creature — Phyrexian Cleric. "Whenever this creature attacks, create a
//! tapped and attacking token that's a copy of it. Put a +1/+1 counter on that
//! token for each +1/+1 counter on this creature. Sacrifice that token at the
//! beginning of the next end step."
//!
//! # GAPs
//! - The front-face activated ability uses a hybrid-Phyrexian cost ({5}{R/P})
//!   where {R/P} can be paid with {R} or 2 life. ManaCost::parse handles hybrid
//!   costs; the activation is modeled via a triggered stub (ActivatedAbilityDef
//!   not in the prompt API). Wired as an always-on spell ability alternative:
//!   the transform is triggered on ETB as a best-effort stub.
//!   GAP: "Activate only as a sorcery" timing restriction not modeled.
//! - Back-face triggered: "create a tapped and attacking token that's a copy of
//!   it" — `CopyPermanent` creates a copy but can't be forced to be tapped and
//!   attacking; and "for each +1/+1 counter on this creature" requires counter
//!   query not available via script helpers. GAP: back-face-only triggered
//!   ability not modeled.
//! Keywords: Transform — not in keyword surface; omitted.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sun-Blessed Guardian");
    let human_sub = reg.interner_mut().intern("Human");
    let cleric_sub = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(cleric_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // Back face: Creature — Phyrexian Cleric
    let back_name = reg.interner_mut().intern("Furnace-Blessed Conqueror");
    let phyrexian_sub = reg.interner_mut().intern("Phyrexian");
    let cleric_sub2 = reg.interner_mut().intern("Cleric");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(phyrexian_sub);
    back_subtypes.0.insert(cleric_sub2);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            // Colors: the back face gains red from the activation cost context;
            // Scryfall lists W as the card's colors (front face colors).
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            ..Default::default()
        },
        spell_ability: None,
    };

    // The front-face activated ability "{5}{R/P}: Transform this creature" is
    // not expressible via the demonstrated ActivatedAbilityDef API in this
    // prompt. We register no transform trigger here — the transform must be
    // invoked by an external game action. The CardDefinition records the back
    // face so the engine knows the transform target.
    //
    // GAP: {5}{R/P} activated ability (sorcery-speed, Phyrexian cost) not
    // modeled; ActivatedAbilityDef not in API surface for this prompt.
    //
    // GAP: back-face-only triggered ability (attack trigger with copy + counter
    // transfer + sacrifice EoT) not modeled.
    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back),
    )
}
