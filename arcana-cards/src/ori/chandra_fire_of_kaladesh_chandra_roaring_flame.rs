//! Chandra, Fire of Kaladesh // Chandra, Roaring Flame (transforming DFC, layout "transform").
//!
//! Front face — Legendary Creature — Human Shaman, 2/2, red:
//!   Whenever you cast a red spell, untap Chandra. (front-only)
//!   {T}: Chandra deals 1 damage to target player or planeswalker. If Chandra has dealt
//!        3 or more damage this turn, exile her, then return her to the battlefield
//!        transformed under her owner's control. (front-only)
//!
//! Back face — Legendary Planeswalker — Chandra, starting loyalty 4:
//!   +1: Chandra deals 2 damage to target player or planeswalker.
//!   -2: Chandra deals 2 damage to target creature.
//!   -7: Chandra deals 6 damage to each opponent. Each player dealt damage this way gets
//!       an emblem with "At the beginning of your upkeep, this emblem deals 3 damage to you."
//!
//! GAPs:
//! - The {T} ability's "If Chandra has dealt 3 or more damage this turn, ... transform"
//!   rider: no PendingTrigger/script accessor exposes damage a source has dealt this turn,
//!   so the conditional transform can't be gated. Only the 1 damage is emitted.
//! - "player or planeswalker"-only target filter doesn't exist; using any_target (over-broad,
//!   also permits creatures).
//! - Planeswalker loyalty abilities are not an expressible ability shape in this catalog, so
//!   the back face's +1 / -2 / -7 abilities are GAPed (the face exists with loyalty 4 but its
//!   activated loyalty abilities, including the -7 emblem, are not wired).

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationCost, ActivationContext, ActivationZone, CardDefinition,
    CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, ObjectOrPlayer, TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chandra, Fire of Kaladesh");
    let human = reg.interner_mut().intern("Human");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(shaman);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // Back face — Legendary Planeswalker — Chandra, loyalty 4.
    let back_name = reg.interner_mut().intern("Chandra, Roaring Flame");
    let chandra_sub = reg.interner_mut().intern("Chandra");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(chandra_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red(),
            types: TypeLine::PLANESWALKER.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            loyalty: Some(4),
            // GAP: planeswalker loyalty abilities (+1 / -2 / -7 incl. the emblem) are
            //   not an expressible ability shape; face exists with loyalty only.
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front (face 0): Whenever you cast a red spell, untap Chandra.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter::new().with_colors(ColorSet::red())),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: untap_chandra,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_trigger_face_gate(1, 0)
            // Front (face 0): {T}: Chandra deals 1 damage to target player or planeswalker.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Chandra deals 1 damage to target player or planeswalker. If \
                       Chandra has dealt 3 or more damage this turn, exile her, then return \
                       her to the battlefield transformed under her owner's control."
                    .into(),
                cost: ActivationCost {
                    tap: true,
                    ..ActivationCost::default()
                },
                // GAP: no "player or planeswalker"-only target filter; any_target is over-broad.
                target_requirements: vec![TargetRequirement::any_target()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: Some(0),
                effect: deal_one_damage,
            }),
    )
}

fn untap_chandra(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Untap { target: trig.source }]
}

fn deal_one_damage(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let dt = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
        _ => return Vec::new(),
    };
    // GAP: "If Chandra has dealt 3 or more damage this turn, exile her, then return her
    //   transformed" — no accessor exposes damage a source has dealt this turn, so the
    //   conditional transform cannot be gated. Only the 1 damage is emitted.
    vec![Effect::DealDamage {
        source: ctx.source,
        target: dt,
        amount: 1,
    }]
}
