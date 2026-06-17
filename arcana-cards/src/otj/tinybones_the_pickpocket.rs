//! Tinybones, the Pickpocket — `{B}` 1/1 Legendary Skeleton Rogue
//! with Deathtouch. "Whenever Tinybones deals combat damage to a
//! player, you may cast target nonland permanent card from that
//! player's graveyard, and mana of any type can be spent to cast that
//! spell."
//!
//! Deathtouch is a base keyword. The combat-damage-to-a-player trigger
//! is wired.
//!
//! GAP: casting a nonland permanent card from the damaged player's
//! graveyard (with any-type mana) is not expressible — there is no
//! cast-from-opponent's-graveyard Effect — so the resolver GAPs.
//! NOTE: the trigger uses a broad "creature you control" source_filter
//! (the source-is-Tinybones restriction is not separately expressed);
//! immaterial since the effect is GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tinybones, the Pickpocket");
    let skeleton = reg.interner_mut().intern("Skeleton");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(skeleton);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter::creature()
                    .controlled_by(ControllerConstraint::You),
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: cast_from_graveyard,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn cast_from_graveyard(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may cast target nonland permanent card from that
    // player's graveyard, mana of any type" has no expressible Effect
    // (no cast-from-another-player's-graveyard primitive).
    Vec::new()
}
