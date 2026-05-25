//! Kami of Tattered Shoji — `{4}{W}` 2/5 white Spirit. "Whenever you
//! cast a Spirit or Arcane spell, this creature gains flying until end
//! of turn."
//!
//! GAP: the trigger filter only matches the *Spirit* creature subtype;
//! the *Arcane* spell-subtype half of the disjunction isn't expressible
//! with the demonstrated `ObjectFilter` surface (no subtype-OR helper,
//! no Arcane spell-type filter). The trigger therefore under-fires on
//! Arcane non-Spirit spells.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kami of Tattered Shoji");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    // Built at register time using the already-interned "Spirit" subtype.
    let spirit_filter = script::subtype_filter(reg, "Spirit");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: filter covers Spirit spells only — the Arcane
                // subtype branch of "Spirit or Arcane spell" is not
                // expressible with the available ObjectFilter API.
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(spirit_filter),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: grant_flying_eot,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// Grant flying to this Kami until end of turn.
fn grant_flying_eot(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::GrantKeyword {
        target: trig.source,
        keyword: KeywordAbility::Flying,
        duration: Duration::EndOfTurn,
    }]
}
