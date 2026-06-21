//! Mangara, the Diplomat — `{3}{W}` 2/4 Legendary Human Cleric with Lifelink.
//! "Whenever an opponent attacks with creatures, if two or more of those
//!  creatures are attacking you and/or planeswalkers you control, draw a
//!  card."
//! "Whenever an opponent casts their second spell each turn, draw a card."
//!
//! GAP: ability 1's gate ("two or more of those creatures attacking you")
//!      is not expressible — there is no attacker-count condition — and the
//!      "attacks with creatures" batch trigger has no matching variant; the
//!      closest (`CreatureAttacks`) fires per attacker. To avoid an
//!      over-firing draw-per-attacker, the effect is GAP'd.
//! GAP: ability 2's "their second spell each turn" gate is not expressible
//!      with the available `conditions::` intervening-if helpers; firing on
//!      every opponent spell would be materially wrong, so the effect is
//!      GAP'd. The triggers are kept for structural fidelity.

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
    let name = reg.interner_mut().intern("Mangara, the Diplomat");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
                },
                intervening_if: None,
                effect: gapped_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::Opponent,
                },
                intervening_if: None,
                effect: gapped_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn gapped_draw(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: the gating condition (two attackers / opponent's second spell) is
    //      not expressible; firing unconditionally would be materially wrong.
    Vec::new()
}
