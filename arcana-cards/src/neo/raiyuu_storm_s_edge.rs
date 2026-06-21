//! Raiyuu, Storm's Edge — `{2}{R}{W}` 3/3 Legendary Creature — Human Samurai.
//!
//! Oracle:
//! * First strike
//! * Whenever a Samurai or Warrior you control attacks alone, untap it. If it's
//!   the first combat phase of the turn, there is an additional combat phase
//!   after this phase.
//!
//! The trigger fires on `AttacksAlone` filtered to Samurai-or-Warrior you
//! control; the sole attacker (`trig.lone_attacker()`) is untapped, and an
//! additional combat phase is added. The "if it's the first combat phase"
//! guard is a fidelity GAP — there is no first-combat-phase predicate, so
//! the extra combat phase is granted unconditionally (the engine has no way
//! to count the current combat phases).

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Raiyuu, Storm's Edge");
    let human = reg.interner_mut().intern("Human");
    let samurai = reg.interner_mut().intern("Samurai");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(samurai);

    // "a Samurai or Warrior you control" — subtype-OR filter.
    let attacker_filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_subtypes_any(vec![samurai, warrior]);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::FirstStrike],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::AttacksAlone { filter: attacker_filter },
                intervening_if: None,
                effect: untap_and_extra_combat,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn untap_and_extra_combat(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = Vec::new();
    if let Some(id) = trig.lone_attacker() {
        effects.push(Effect::Untap { target: id });
    }
    // GAP (fidelity): "If it's the first combat phase of the turn" — no
    // first-combat-phase predicate; the extra combat phase is added
    // unconditionally.
    effects.push(Effect::AdditionalCombatPhase);
    effects
}
