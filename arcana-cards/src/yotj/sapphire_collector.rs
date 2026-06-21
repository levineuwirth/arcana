//! Sapphire Collector — `{2}{R}` 3/3 Human Mercenary.
//!
//! Oracle:
//! * Prowess — not a usable KeywordAbility variant. GAP'd.
//! * "When you cast your second noncreature spell in a turn, conjure a card
//!   named Mox Sapphire into your hand. This ability triggers only once." —
//!   Conjure is not modeled (Arena-only mechanic; no Effect::Conjure). The
//!   "second spell in a turn" gate is also not applied. Trigger registered with
//!   a noncreature SpellCast filter; effect GAP'd.
//! * "{2}{U}: Target instant or sorcery card in your graveyard gains flashback
//!   until end of turn. The flashback cost is equal to its mana cost." —
//!   granting flashback to a graveyard card is not expressible (no Flashback
//!   keyword grant). Activated ability registered; effect GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sapphire Collector");
    let human = reg.interner_mut().intern("Human");
    let mercenary = reg.interner_mut().intern("Mercenary");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(mercenary);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: Prowess is not a usable KeywordAbility variant.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(
                        ObjectFilter::new().without_types(TypeLine::CREATURE.into()),
                    ),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: conjure_mox,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::OncePerGame,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{U}: Target instant or sorcery card in your graveyard \
                       gains flashback until end of turn. The flashback cost is \
                       equal to its mana cost.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{U}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::new().with_types_any(TypeLine(
                            TypeLine::INSTANT | TypeLine::SORCERY,
                        )),
                    },
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: grant_flashback,
            }),
    )
}

fn conjure_mox(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Conjure not modeled (Arena-only mechanic; would need registry-by-name
    // lookup in Effect::execute). The "second noncreature spell in a turn" gate
    // is also not applied.
    Vec::new()
}

fn grant_flashback(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: granting flashback to a graveyard card is not expressible (no
    // Flashback keyword grant / cast-from-graveyard permission Effect).
    Vec::new()
}
