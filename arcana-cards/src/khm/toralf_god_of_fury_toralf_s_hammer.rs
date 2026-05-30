//! Toralf, God of Fury // Toralf's Hammer
//!
//! Front face: `{2}{R}{R}` Legendary Creature — God, 5/4.
//!   Trample.
//!   Whenever a creature or planeswalker an opponent controls is dealt excess noncombat
//!   damage, Toralf deals damage equal to the excess to any target other than that permanent.
//!
//! Back face: `{1}{R}` Legendary Artifact — Equipment.
//!   Equipped creature has "{1}{R}, {T}, Unattach Toralf's Hammer: It deals 3 damage to any
//!   target. Return Toralf's Hammer to its owner's hand."
//!   Equipped creature gets +3/+0 as long as it's legendary.
//!   Equip {1}{R}
//!
//! GAP: Front trigger "excess noncombat damage" — no accessor for excess damage in script;
//!      trigger registered but effect is empty (dynamic amount not computable).
//! GAP: "deals damage equal to the excess" — DamageDealt trigger doesn't expose excess amount.
//! GAP: Back-face-only activated abilities (Equip, granted abilities) not auto-installed.
//! GAP: Equipment grant ("equipped creature has", "gets +3/+0 as long as legendary") —
//!      continuous static equipment effects not modeled.
//! GAP: Keyword "Equip" not separately in KeywordAbility (equip is an activated ability shape).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Toralf, God of Fury");
    let god_sub = reg.interner_mut().intern("God");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(god_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    // Back face: Toralf's Hammer — Legendary Artifact — Equipment
    let back_name = reg.interner_mut().intern("Toralf's Hammer");
    let equipment_sub = reg.interner_mut().intern("Equipment");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(equipment_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
            colors: ColorSet::red(),
            types: TypeLine::ARTIFACT.into(),
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            subtypes: back_subtypes,
            keywords: vec![],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_mdfc_back(back)
            // Triggered: whenever a creature or planeswalker an opponent controls is dealt
            // excess noncombat damage, Toralf deals damage equal to the excess to any target.
            // GAP: excess noncombat damage amount not accessible via script; effect is empty.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::new(),
                    target_filter: TargetFilter::Permanent(
                        ObjectFilter::new()
                            .with_types_any(TypeLine(TypeLine::CREATURE | TypeLine::PLANESWALKER))
                            .controlled_by(ControllerConstraint::Opponent),
                    ),
                    combat_only: false,
                },
                intervening_if: None,
                effect: excess_damage_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::any_target()],
            }),
        // GAP: back-face-only activated abilities (Equip {1}{R}, granted activated ability)
        //      not auto-installed on the back face — back-face activated ability engine debt.
    )
}

fn excess_damage_trigger(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "damage equal to the excess" — excess noncombat damage amount not accessible
    // via any script:: helper; cannot compute dynamic damage. Effect omitted.
    Vec::new()
}
