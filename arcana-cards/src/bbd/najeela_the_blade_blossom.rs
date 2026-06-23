//! Najeela, the Blade-Blossom — `{2}{R}` 3/2 Legendary Human Warrior.
//!
//! * Whenever a Warrior attacks, you may have its controller create a 1/1
//!   white Warrior creature token that's tapped and attacking.
//! * `{W}{U}{B}{R}{G}`: Untap all attacking creatures. They gain trample,
//!   lifelink, and haste until end of turn. After this phase, there is an
//!   additional combat phase. Activate only during combat.
//!
//! The attack trigger uses `CreateTokenTappedAttacking`. The activated
//! ability is GAP'd: there is no documented ObjectFilter refinement for
//! "attacking creatures" to sweep, and no primitive for an additional
//! combat phase.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Najeela, the Blade-Blossom");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    let warrior_filter = script::subtype_filter(reg, "Warrior");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: warrior_filter,
                },
                intervening_if: None,
                effect: warrior_attacks_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{W}{U}{B}{R}{G}: Untap all attacking creatures. They gain trample, \
                       lifelink, and haste until end of turn. After this phase, there is an \
                       additional combat phase. Activate only during combat."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{W}{U}{B}{R}{G}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: untap_attackers_extra_combat,
            }),
    )
}

/// The attacking Warrior's controller may create a 1/1 white Warrior token
/// that's tapped and attacking.
fn warrior_attacks_token(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let attacker = trig.attacking_creature().unwrap_or(trig.source);
    let controller = state
        .objects
        .get(attacker)
        .map(|o| o.controller)
        .unwrap_or(trig.controller);
    let warrior = reg.interner().lookup("Warrior").unwrap_or_default();
    let mut subs = SubtypeSet::default();
    subs.0.insert(warrior);
    vec![Effect::CreateTokenTappedAttacking {
        controller,
        token: TokenDefinition {
            name: warrior,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes: subs,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

fn untap_attackers_extra_combat(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: no documented ObjectFilter refinement for "attacking creatures" to
    // sweep with Untap + grant trample/lifelink/haste, and no primitive for an
    // additional combat phase. Activation cost (the WUBRG drain) is faithful.
    Vec::new()
}
