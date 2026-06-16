//! Tadeas, Juniper Ascendant — `{2}{G}{W}` 1/3 Legendary Human Monk.
//! Reach; "Tadeas has hexproof unless it's attacking."
//! "Whenever a creature you control with reach attacks, untap it and it
//! can't be blocked by creatures with greater power this combat."
//! "Whenever one or more creatures you control deal combat damage to a
//! player, draw a card."
//!
//! Reach is a usable keyword. The conditional-hexproof static is GAP'd
//! (a conditional static). The reach-attack trigger untaps the attacker
//! (the "can't be blocked by greater power" clause is GAP'd). The
//! combat-damage draw fires per creature rather than once for "one or
//! more" (GAP).

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
    let name = reg.interner_mut().intern("Tadeas, Juniper Ascendant");
    let human = reg.interner_mut().intern("Human");
    let monk = reg.interner_mut().intern("Monk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(monk);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };
    // GAP: "Tadeas has hexproof unless it's attacking" — conditional static.
    reg.register(
        CardDefinition::new(name, chars)
            // GAP: "can't be blocked by creatures with greater power this
            // combat" clause not expressible; only the untap is emitted.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You)
                        .with_keyword(KeywordAbility::Reach),
                },
                intervening_if: None,
                effect: untap_attacker,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // GAP: "one or more creatures" should fire once per combat;
            // DamageDealt fires per damaging creature.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: draw_one,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn untap_attacker(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(id) = trig.attacking_creature() else { return Vec::new(); };
    vec![Effect::Untap { target: id }]
}

fn draw_one(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards { player: trig.controller, count: 1 }]
}
