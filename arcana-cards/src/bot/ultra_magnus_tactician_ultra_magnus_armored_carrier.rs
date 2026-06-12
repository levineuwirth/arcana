//! Ultra Magnus, Tactician // Ultra Magnus, Armored Carrier (transforming DFC, layout "transform")
//!
//! Front face: Ultra Magnus, Tactician — {4}{R}{G}{W} Legendary Artifact Creature — Robot, 7/7 (G/R/W).
//!   More Than Meets the Eye {2}{R}{G}{W}.
//!   Ward {2}.
//!   Whenever Ultra Magnus attacks, you may put an artifact creature card from your hand onto
//!     the battlefield tapped and attacking. If you do, convert Ultra Magnus at end of combat.
//! Back face: Ultra Magnus, Armored Carrier — Legendary Artifact — Vehicle (G/R/W).
//!   Living metal. Haste.
//!   Formidable — Whenever Ultra Magnus attacks, attacking creatures you control gain
//!     indestructible until end of turn. If those creatures have total power 8 or greater,
//!     convert Ultra Magnus.
//!
//! GAP: "More Than Meets the Eye" / "Living metal" / "Formidable" / "Convert" are not in the
//!   usable keyword surface — only Haste and Ward {2} are emitted. The front attack trigger's
//!   "you may put an artifact creature card from your hand onto the battlefield tapped and
//!   attacking" is wired via Effect::PutFromHandOntoBattlefieldTappedAttacking (face-gated to
//!   the front face); its "If you do, convert Ultra Magnus at end of combat" rider stays a GAP
//!   (no if-you-do linkage / end-of-combat-conditional-transform hook). The back Formidable
//!   trigger remains a GAP: it grants indestructible to the dynamic set "attacking creatures
//!   you control" and converts on a total-power threshold.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ultra Magnus, Tactician");
    let robot = reg.interner_mut().intern("Robot");
    let vehicle = reg.interner_mut().intern("Vehicle");

    let mut front_subtypes = SubtypeSet::default();
    front_subtypes.0.insert(robot);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red() | ColorSet::white(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        subtypes: front_subtypes,
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![KeywordAbility::Ward(ManaCost::parse("{2}").expect("valid cost"))],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Ultra Magnus, Armored Carrier");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(vehicle);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::green() | ColorSet::red() | ColorSet::white(),
            types: TypeLine(TypeLine::ARTIFACT),
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            subtypes: back_subtypes,
            keywords: vec![KeywordAbility::Haste],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front face: "Whenever Ultra Magnus attacks, you may put an
            // artifact creature card from your hand onto the battlefield
            // tapped and attacking." GAP: "If you do, convert Ultra
            // Magnus at end of combat" — no if-you-do linkage or
            // end-of-combat-conditional-transform hook.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: on_attack_put_artifact_creature,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_trigger_face_gate(1, 0),
    )
}

fn on_attack_put_artifact_creature(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::PutFromHandOntoBattlefieldTappedAttacking {
        player: trig.controller,
        filter: ObjectFilter::new()
            .with_types(TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE)),
    }]
}
