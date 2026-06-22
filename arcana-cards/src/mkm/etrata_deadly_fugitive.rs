//! Etrata, Deadly Fugitive — `{1}{U}{B}` 1/4 Legendary Vampire Assassin with Deathtouch.
//! "Face-down creatures you control have '{2}{U}{B}: Turn this creature face up.
//!  If you can't, exile it, then you may cast the exiled card without paying its
//!  mana cost.'"
//! "Whenever an Assassin you control deals combat damage to an opponent, cloak
//!  the top card of that player's library."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Etrata, Deadly Fugitive");
    let vampire = reg.interner_mut().intern("Vampire");
    let assassin = reg.interner_mut().intern("Assassin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(assassin);

    // GAP: keyword Cloak — not in the implemented keyword surface.
    // GAP: static "Face-down creatures you control have '{2}{U}{B}: Turn this
    // creature face up …'" — granting an activated ability to other face-down
    // creatures (disguise/cloak machinery) is not expressible.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // GAP: effect "cloak the top card of that player's library" — no
            // Cloak effect primitive. Trigger condition wired faithfully
            // (Assassin you control deals combat damage to a player).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: script::subtype_filter(reg, "Assassin")
                        .controlled_by(ControllerConstraint::You),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: cloak_top_card,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn cloak_top_card(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "cloak the top card of that player's library" — no Cloak primitive.
    Vec::new()
}
