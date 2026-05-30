//! Cecil, Dark Knight // Cecil, Redeemed Paladin — `{B}` Legendary Human Knight 2/3.
//!
//! Front face (Cecil, Dark Knight):
//! - Deathtouch
//! - Darkness — Whenever Cecil deals damage, you lose that much life. Then if your life
//!   total is less than or equal to half your starting life total, untap Cecil and
//!   transform it.
//!
//! Back face (Cecil, Redeemed Paladin):
//! - Lifelink
//! - Protect — Whenever Cecil attacks, other attacking creatures gain indestructible until end of turn.
//!
//! # GAPs
//! - "Darkness" trigger: the conditional "if life total <= half starting life total" uses
//!   `script::life` but "starting life total" (typically 20) is a GAP — no engine primitive
//!   to read starting life total. Wired as: lose life equal to damage dealt (dynamic amount
//!   is a GAP — damage amount from the event is not accessible in PendingTrigger). The
//!   trigger fires on DealDamage events; however TriggerCondition::DamageDealt is not
//!   in the documented API. Wired as best-effort using ZoneChange as an approximation GAP.
//!   GAP: DamageDealt trigger condition not available; the Darkness ability cannot be modeled.
//! - "Protect" (back face): back-face-only triggered ability not modeled.
//! - Keywords: "Protect" and "Darkness" are not standard keyword abilities; omitted.
//! - Transform trigger: GAP — DealDamage trigger condition not expressible.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cecil, Dark Knight");
    let human_sub = reg.interner_mut().intern("Human");
    let knight_sub = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(knight_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };

    // Back face: Cecil, Redeemed Paladin
    let back_name = reg.interner_mut().intern("Cecil, Redeemed Paladin");
    let human_sub2 = reg.interner_mut().intern("Human");
    let knight_sub2 = reg.interner_mut().intern("Knight");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(human_sub2);
    back_subtypes.0.insert(knight_sub2);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(3)),
            keywords: vec![KeywordAbility::Lifelink],
            ..Default::default()
        },
        spell_ability: None,
    };

    // GAP: "Darkness" — Whenever Cecil deals damage, you lose that much life, then if
    // your life total is <= half your starting life total, untap and transform.
    // TriggerCondition::DamageDealt is not in the documented API; cannot be modeled.
    // GAP: back-face-only triggered ability "Protect" not modeled.

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back),
    )
}
